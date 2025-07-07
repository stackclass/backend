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

use std::path::PathBuf;

use codecraft::schema::{self, Difficulty, Status};

#[test]
fn test_parse_course() {
    // read course from sample dir.
    let path = PathBuf::from("samples/build-your-own-interpreter");
    let course = schema::parse(&path).unwrap();

    // test course
    assert_eq!(course.slug, "interpreter");
    assert_eq!(course.name, "Build your own Interpreter");
    assert_eq!(course.short_name, "Interpreter");
    assert!(matches!(course.release_status, Status::Beta));

    // test stages
    let stages = course.stages;
    assert!(!stages.is_empty());
    let (key, stage) = stages.first().unwrap();
    assert_eq!(key, "01-ry8-scanning-empty-file");
    assert_eq!(stage.slug, "ry8");
    assert_eq!(stage.name, "Scanning: Empty file");
    assert_eq!(stage.difficulty, Difficulty::VeryEasy);
    assert!(stage.description.starts_with("In this stage"));
    assert!(stage.instruction.starts_with("Before start"));

    // test stage solution
    let solution = stage.solution.as_ref().unwrap();
    assert!(solution.starts_with("Since this is the first stage"));

    // test extensions
    let extensions = course.extensions;
    assert!(extensions.is_some());
    let extensions = extensions.unwrap();

    let mut iter = extensions.keys();
    assert_eq!(iter.next().unwrap(), "parsing-expressions");
    assert_eq!(iter.next().unwrap(), "evaluating-expressions");

    // test extension stages
    let parsing_expr = extensions.get("parsing-expressions").unwrap();
    let parsing_stages = &parsing_expr.stages;
    assert!(!parsing_stages.is_empty());

    // test extension stage
    let (key, stage) = parsing_stages.first().unwrap();
    assert_eq!(key, "01-sc2-booleans-and-nil");
    assert_eq!(stage.slug, "sc2");
    assert_eq!(stage.name, "Booleans & Nil");
    assert_eq!(stage.difficulty, Difficulty::Hard);
    assert!(stage.description.starts_with("In this stage"));
    assert!(stage.instruction.starts_with("In this stage"));
}
