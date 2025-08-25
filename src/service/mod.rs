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

mod course;
mod extension;
mod pipeline;
mod registry;
mod repository;
mod stage;
mod storage;

// Re-exports
pub use course::CourseService;
pub use extension::ExtensionService;
pub use pipeline::{PipelineCleanupGuard, PipelineService};
pub use registry::RegistryService;
pub use repository::RepoService;
pub use stage::StageService;
pub use storage::{StorageError, StorageService};
