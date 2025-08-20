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

use std::{collections::HashMap, sync::Arc, time::Duration};

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{
    Api,
    api::{ApiResource, DynamicObject, GroupVersionKind, PostParams},
};
use serde_json::json;
use tokio::time::interval;
use tracing::debug;

use crate::{
    config::Config,
    context::Context,
    errors::{ApiError, Result},
    repository::StageRepository,
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
    pub async fn trigger(&self, owner: &str, repo: &str, current_stage_slug: &str) -> Result<()> {
        debug!("Triggering PipelineRun for repository: {owner}/{repo}");

        let resource = self.generate(owner, repo, current_stage_slug).await?;
        self.api().create(&PostParams::default(), &resource).await?;

        Ok(())
    }

    /// Watches a PipelineRun and invokes the callback only on success.
    pub async fn watch<F, Fut, T>(&self, owner: &str, repo: &str, callback: F) -> Result<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, ApiError>> + Send + 'static,
    {
        let name = pipeline_name(owner, repo);
        let api = self.api();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                match Self::status(&api, &name).await {
                    Ok(true) => {
                        // Pipeline succeeded; invoke the callback
                        callback().await.ok();
                        break;
                    }
                    Ok(false) => continue, // Pipeline still running
                    Err(_) => break,       // Pipeline failed or error occurred
                }
            }
        });

        Ok(())
    }

    /// Checks the status of a PipelineRun.
    async fn status(api: &Api<DynamicObject>, name: &str) -> Result<bool> {
        debug!("Checking status for PipelineRun: {}", name);

        let resource = api.get(name).await?;

        if let Some(conditions) = resource.data.pointer("/status/conditions") {
            let conditions: Vec<Condition> =
                serde_json::from_value(json!(conditions)).map_err(ApiError::SerializationError)?;

            #[rustfmt::skip] // just for better format :)
            return Ok(conditions.iter().any(|condition| {
                condition.type_ == "Succeeded" && condition.status == "True"
            }));
        }

        Ok(false)
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
    async fn generate(&self, owner: &str, repo: &str, stage: &str) -> Result<DynamicObject> {
        let Config { git_server_endpoint, docker_registry_endpoint, .. } = &self.ctx.config;

        // Build test cases JSON value from all stages up to the current stage
        let stages = StageRepository::find_stages_until(&self.ctx.database, repo, stage).await?;
        let slugs: Vec<&str> = stages.iter().map(|stage| stage.slug.as_str()).collect();
        let cases = build_test_cases_json(&slugs);

        // Define the pipeline run name
        let name = pipeline_name(owner, repo);

        // Define params as a HashMap and then convert it to JSON value
        let params = HashMap::from([
            ("REPO_URL", format!("{git_server_endpoint}/{owner}/{repo}.git")),
            ("REPO_REF", "main".to_string()),
            ("COURSE_IMAGE", format!("{docker_registry_endpoint}/library/{owner}-{repo}:latest")),
            ("TESTER_IMAGE", format!("ghcr.io/stackclass/{repo}-tester")),
            (
                "TEST_IMAGE",
                format!("{docker_registry_endpoint}/library/{owner}-{repo}-test:latest"),
            ),
            ("COMMAND", format!("/app/{repo}-tester")),
            ("TEST_CASES_JSON", cases),
            ("DEBUG_MODE", "false".to_string()),
            ("TIMEOUT_SECONDS", "15".to_string()),
            ("SKIP_ANTI_CHEAT", "false".to_string()),
        ]);

        // Render a PipelineRun resource using the provided name and parameters
        resource(name, params).map_err(ApiError::SerializationError)
    }
}

#[inline]
fn pipeline_name(owner: &str, repo: &str) -> String {
    format!("{owner}-{repo}-test-pipeline")
}

/// Builds a JSON string representing test cases from a list of slugs.
fn build_test_cases_json(slugs: &[&str]) -> String {
    let mut test_cases = Vec::new();
    for (index, slug) in slugs.iter().enumerate() {
        test_cases.push(serde_json::json!({
            "slug": slug,
            "log_prefix": format!("test-{}", index + 1),
            "title": format!("Stage #{}: {}", index + 1, slug),
        }));
    }
    serde_json::to_string(&test_cases).unwrap()
}

/// Creates a new DynamicObject representing a Tekton PipelineRun resource.
fn resource(
    name: String,
    params: HashMap<&'static str, String>,
) -> Result<DynamicObject, serde_json::Error> {
    let params: serde_json::Value =
        params.into_iter().map(|(name, value)| json!({"name": name, "value": value})).collect();

    let resource = json!({
      "apiVersion": "tekton.dev/v1",
      "kind": "PipelineRun",
      "metadata": {
        "name": name
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
                    "storage": "1Gi"
                  }
                }
              }
            }
          },
          {
            "name": "docker-config",
            "secret": {
              "secretName": "docker-config"
            }
          }
        ]
      }
    });

    serde_json::from_value(resource)
}
