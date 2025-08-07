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

use std::{collections::HashMap, sync::Arc};

use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::{
    Api,
    api::{ApiResource, DynamicObject, GroupVersionKind, PostParams},
};
use serde_json::json;
use tracing::debug;

use crate::{
    config::Config,
    context::Context,
    errors::{ApiError, Result},
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
    pub async fn trigger(&self, owner: &str, repo: &str) -> Result<()> {
        debug!("Triggering PipelineRun for repository: {owner}/{repo}");

        let resource = self.generate(owner, repo)?;
        self.api().create(&PostParams::default(), &resource).await?;

        Ok(())
    }

    /// Checks the status of a PipelineRun.
    pub async fn status(&self, pipeline_name: &str) -> Result<bool> {
        debug!("Checking status for PipelineRun: {}", pipeline_name);

        let resource = self.api().get(pipeline_name).await?;

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

    /// Generates a PipelineRun manifest for the given repository.
    fn generate(&self, owner: &str, repo: &str) -> Result<DynamicObject> {
        let Config { git_server_endpoint, docker_registry_endpoint, .. } = &self.ctx.config;

        // Define the pipeline run name
        let name = format!("{owner}-{repo}-test-pipeline");

        // Define params as a HashMap and then convert it to JSON value
        let params = HashMap::from([
            ("REPO_URL", format!("{git_server_endpoint}/{owner}/{repo}.git")),
            ("REPO_REF", "main".to_string()),
            ("COURSE_IMAGE", format!("{docker_registry_endpoint}/{owner}/{repo}:latest")),
            ("TESTER_IMAGE", format!("ghcr.io/stackclass/{repo}-tester")),
            ("TEST_IMAGE", format!("{docker_registry_endpoint}/{owner}/{repo}-test:latest")),
            ("COMMAND", "/app/interpreter-tester".to_string()),
        ]);
        let params: serde_json::Value =
            params.into_iter().map(|(name, value)| json!({"name": name, "value": value})).collect();

        // Define workspaces as a JSON value
        let workspaces = json!([
          {
            "name": "context",
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
        ]);

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
            "params": params,
            "workspaces": workspaces
          }
        });

        serde_json::from_value(resource).map_err(ApiError::SerializationError)
    }
}
