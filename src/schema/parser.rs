// Copyright (c) wangeguo. All rights reserved.
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

use indexmap::IndexMap;
use std::{fs, path::Path, str::FromStr};
use thiserror::Error;
use walkdir::WalkDir;

use crate::schema::{Course, ExtensionMap, ExtensionSet, Solution, Stage};

/// Errors that can occur during course parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yml::Error),
    #[error("Invalid course structure: {0}")]
    Structure(String),
    #[error("Validation failed: {0}")]
    Validation(String),
}

/// Parser for course structure and content
pub struct CourseParser;

impl CourseParser {
    /// Parse entire course including stages and extensions
    pub fn parse<P: AsRef<Path>>(&self, path: P) -> Result<Course, ParseError> {
        let path = path.as_ref();

        let mut course = self.parse_course(path)?;
        course.stages = self.parse_stages(path.join("stages"))?;
        course.extensions = self.parse_extensions(path)?;

        Ok(course)
    }

    /// Parse course metadata from course.yml
    fn parse_course(&self, path: &Path) -> Result<Course, ParseError> {
        let content = fs::read_to_string(path.join("course.yml"))?;
        Course::from_str(&content).map_err(Into::into)
    }

    /// Parse all stages from stages directory
    fn parse_stages<P: AsRef<Path>>(
        &self,
        stages_dir: P,
    ) -> Result<IndexMap<String, Stage>, ParseError> {
        if !stages_dir.as_ref().exists() {
            return Err(ParseError::Structure("stages directory not found".into()));
        }

        let mut stages = IndexMap::new();

        // Process each stage directory
        for entry in fs::read_dir(stages_dir)? {
            let entry = entry?;
            let stage_dir = entry.path();

            if stage_dir.is_dir() {
                let file_name = stage_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| ParseError::Structure("Invalid stage directory name".into()))?
                    .to_string();
                stages.insert_sorted(file_name, self.parse_stage(&stage_dir)?);
            }
        }

        Ok(stages)
    }

    /// Parse single stage including instruction and solution
    fn parse_stage<P: AsRef<Path>>(&self, stage_dir: P) -> Result<Stage, ParseError> {
        let path = stage_dir.as_ref();

        let meta_content = fs::read_to_string(path.join("stage.yml"))?;
        let mut stage = Stage::from_str(&meta_content)?;

        stage.instruction = fs::read_to_string(path.join("instruction.md"))?;
        stage.solution = self.parse_solution(path.join("solution"))?;

        Ok(stage)
    }

    /// Parse solution including explanation and patches
    fn parse_solution<P: AsRef<Path>>(&self, sln_dir: P) -> Result<Option<Solution>, ParseError> {
        let path = sln_dir.as_ref();

        if !path.exists() {
            return Ok(None);
        }

        let explanation_path = path.join("explanation.md");
        if !explanation_path.exists() {
            return Ok(None);
        }

        let explanation = fs::read_to_string(explanation_path)?;
        let mut solution = Solution::new(explanation);

        // Process patch files if patches directory exists
        let patches_dir = path.join("patches");
        if !patches_dir.exists() {
            return Ok(Some(solution));
        }

        for entry in WalkDir::new(&patches_dir).into_iter().filter_map(|e| e.ok()) {
            let patch_path = entry.path();

            if patch_path.is_file() {
                let content = fs::read_to_string(patch_path)?;
                let file_name = patch_path
                    .strip_prefix(&patches_dir)
                    .map_err(|_| {
                        ParseError::Structure("Failed to strip patch directory prefix".into())
                    })?
                    .to_str()
                    .ok_or_else(|| ParseError::Structure("Invalid patch file path".into()))?
                    .trim_end_matches(".diff")
                    .to_string();

                solution.add_patch(file_name, content);
            }
        }

        Ok(Some(solution))
    }

    /// Parse extensions including their stages
    fn parse_extensions<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Option<ExtensionMap>, ParseError> {
        let path = path.as_ref();

        let extensions_path = path.join("extensions.yml");
        if !extensions_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(extensions_path)?;
        let extensions = ExtensionSet::from_str(&content)?;
        let mut extensions: ExtensionMap = extensions.into();

        // Process extension stages if extensions directory exists
        let extensions_dir = path.join("extensions");
        if !extensions_dir.exists() {
            return Ok(Some(extensions));
        }

        for (slug, extension) in extensions.iter_mut() {
            let stages_dir = extensions_dir.join(slug);
            if stages_dir.exists() {
                extension.stages = self.parse_stages(stages_dir)?;
            }
        }

        Ok(Some(extensions))
    }
}
