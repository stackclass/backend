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

use crate::schema::{Course, ExtensionMap, ExtensionSet, Stage};

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

/// Parse entire course including stages and extensions
pub fn parse(path: &Path) -> Result<Course, ParseError> {
    if !path.exists() {
        return Err(ParseError::Structure("Course directory not found".into()));
    }

    let mut course = parse_course(path)?;
    course.stages = parse_stages(&path.join("stages"))?;
    course.extensions = parse_extensions(path)?;

    Ok(course)
}

/// Parse course metadata from course.yml
fn parse_course(path: &Path) -> Result<Course, ParseError> {
    let content = fs::read_to_string(path.join("course.yml"))?;
    Course::from_str(&content).map_err(Into::into)
}

/// Parse all stages from stages directory
fn parse_stages(stages_dir: &Path) -> Result<IndexMap<String, Stage>, ParseError> {
    if !stages_dir.exists() {
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
            stages.insert_sorted(file_name, parse_stage(&stage_dir)?);
        }
    }

    Ok(stages)
}

/// Parse single stage including instruction and solution
fn parse_stage(stage_dir: &Path) -> Result<Stage, ParseError> {
    let meta_content = fs::read_to_string(stage_dir.join("stage.yml"))?;
    let mut stage = Stage::from_str(&meta_content)?;

    stage.instruction = fs::read_to_string(stage_dir.join("instruction.md"))?;

    let sln_path = stage_dir.join("solution.md");
    if sln_path.exists() {
        stage.solution.replace(fs::read_to_string(sln_path)?);
    }

    Ok(stage)
}

/// Parse extensions including their stages
fn parse_extensions(path: &Path) -> Result<Option<ExtensionMap>, ParseError> {
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
            extension.stages = parse_stages(&stages_dir)?;
        }
    }

    Ok(Some(extensions))
}
