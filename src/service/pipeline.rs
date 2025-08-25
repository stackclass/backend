// Copyright (c) The StackClass Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use kube::{
    Api,
    api::{ApiResource, DynamicObject, GroupVersionKind, PostParams},
};
use serde_json::{Error as JsonError, Value, json};
use tracing::debug;
use uuid::Uuid;

use crate::{
    context::Context,
    errors::{ApiError, Result},
    repository::StageRepository,
    utils::{crypto, url},
};

/// A service for managing Tekton PipelineRun resources.
pub struct PipelineService {
    ctx: Arc<Context>,
}

impl PipelineService {
    /// Creates a new instance with the provided context.
    pub fn new(ctx: Arc<Context>) -> Self {
        PipelineService { ctx }
    }

    /// Triggers a Tekton PipelineRun for the given repository.
    pub async fn trigger(&self, repo: &str, course: &str, stage: &str) -> Result<()> {
        debug!("Triggering PipelineRun for repository: {course} - {repo}");

        let resource = self.generate(repo, course, stage).await?;
        self.api().create(&PostParams::default(), &resource).await?;

        Ok(())
    }

    #[inline]
    fn api(&self) -> Api<DynamicObject> {
        let gvk = GroupVersionKind::gvk("tekton.dev", "v1", "PipelineRun");
        Api::namespaced_with(
            self.ctx.k8s.clone(),
            self.ctx.config.namespace.as_ref(),
            &ApiResource::from_gvk(&gvk),
        )
    }

    /// Generates a PipelineRun resource for the given repository.
    async fn generate(&self, repo: &str, course: &str, stage: &str) -> Result<DynamicObject> {
        let name = Uuid::now_v7().to_string();

        // Define labels for identification
        let labels = vec![
            ("stackclass.dev/repo", repo.to_string()),
            ("stackclass.dev/course", course.to_string()),
            ("stackclass.dev/stage", stage.to_string()),
        ];

        // Build test cases JSON value from all stages up to the current stage
        let stages = StageRepository::find_stages_until(&self.ctx.database, course, stage).await?;
        let slugs: Vec<&str> = stages.iter().map(|stage| stage.slug.as_str()).collect();
        let cases = build_test_cases_json(&slugs);

        // Configuration values for the PipelineRun
        let git_endpoint = &self.ctx.config.git_server_endpoint;
        let registry = url::hostname(&self.ctx.config.docker_registry_endpoint)?;
        let org = &self.ctx.config.namespace;

        // Construct the webhook URL for Tekton to send notifications
        let webhook_endpoint = &self.ctx.config.webhook_endpoint;
        let webhook_url = format!("{webhook_endpoint}/v1/webhooks/tekton");

        // Generate HMAC signature for webhook authentication
        let auth_secret = &self.ctx.config.auth_secret;
        let payload = format!("{}{}{}", repo, course, stage);
        let secret = crypto::hmac_sha256_sign(&payload, auth_secret).map_err(|e| {
            ApiError::InternalError(format!("Failed to generate HMAC signature: {}", e))
        })?;

        // Define parameters for the PipelineRun
        let params = vec![
            ("REPO_URL", format!("{git_endpoint}/{org}/{repo}.git")),
            ("COURSE_IMAGE", format!("{registry}/{org}/{repo}:latest")),
            ("TESTER_IMAGE", format!("ghcr.io/stackclass/{course}-tester")),
            ("TEST_IMAGE", format!("{registry}/{org}/{repo}-test:latest")),
            ("COMMAND", format!("/app/{course}-tester")),
            ("TEST_CASES_JSON", cases),
            ("WEBHOOK_URL", webhook_url),
            ("REPO", repo.to_string()),
            ("COURSE", course.to_string()),
            ("STAGE", stage.to_string()),
            ("SECRET", secret),
        ];

        // Render a PipelineRun resource with the given name, labels, and params
        resource(&name, labels, params).map_err(ApiError::SerializationError)
    }
}

/// Builds a JSON string representing test cases from a list of slugs.
fn build_test_cases_json(slugs: &[&str]) -> String {
    let mut test_cases = Vec::new();
    for (index, slug) in slugs.iter().enumerate() {
        test_cases.push(json!({
            "slug": slug,
            "log_prefix": format!("test-{}", index + 1),
            "title": format!("Stage #{}: {}", index + 1, slug),
        }));
    }
    serde_json::to_string(&test_cases).unwrap()
}

/// Creates a new DynamicObject representing a Tekton PipelineRun resource.
fn resource<T>(name: &str, labels: T, params: T) -> Result<DynamicObject, JsonError>
where
    T: IntoIterator<Item = (&'static str, String)>,
{
    let labels: Value = labels.into_iter().collect();
    let params: Value = params.into_iter().map(|(k, v)| json!({"name": k, "value": v})).collect();

    let resource = json!({
      "apiVersion": "tekton.dev/v1",
      "kind": "PipelineRun",
      "metadata": {
        "name": name,
        "labels": labels
      },
      "spec": {
        "pipelineRef": {
          "name": "course-test-pipeline"
        },
        "podTemplate": {
            "securityContext": {
              "fsGroup": 65532
            }
        },
        "params": params,
        "workspaces": [
          {
            "name": "shared-workspace",
            "volumeClaimTemplate": {
              "spec": {
                "accessModes": [
                  "ReadWriteOnce"
                ],
                "resources": {
                  "requests": {
                    "storage": "5Gi"
                  }
                }
              }
            }
          },
          {
            "name": "docker-credentials",
            "secret": {
              "secretName": "docker-credentials"
            }
          }
        ]
      }
    });

    serde_json::from_value(resource)
}
