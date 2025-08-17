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

use url::{ParseError, Url};

/// Constructs an authenticated URL by embedding the username and password.
pub fn authenticate(url: &str, username: &str, password: &str) -> Result<String, ParseError> {
    let mut parsed_url = Url::parse(url)?;

    let _ = parsed_url.set_username(username);
    let _ = parsed_url.set_password(Some(password));

    Ok(parsed_url.to_string())
}
