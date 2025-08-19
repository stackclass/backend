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

use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;

/// Database model representing a user entity
#[derive(Debug, FromRow)]
pub struct UserModel {
    /// Unique identifier for each user
    pub id: String,

    /// User's chosen display name
    pub name: String,

    /// User's email address for communication and login
    pub email: String,

    /// Whether the user's email is verified
    pub email_verified: bool,

    /// User's image url
    pub image: Option<String>,

    /// Creation timestamp
    pub created_at: NaiveDateTime,

    /// Last update timestamp
    pub updated_at: NaiveDateTime,
}

impl UserModel {
    /// Creates a new UserModel with the given parameters
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            email_verified: false,
            image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    /// Sets the email_verified field
    pub fn with_email_verified(mut self, verified: bool) -> Self {
        self.email_verified = verified;
        self
    }

    /// Sets the image field
    pub fn with_image(mut self, image: String) -> Self {
        self.image = Some(image);
        self
    }

    /// Returns a normalized version of the user's display name for use as a
    /// username. Currently converts to lowercase and removes spaces.
    ///
    /// @TODO: Use GitHub username in the future.
    pub fn username(&self) -> String {
        self.name.to_ascii_lowercase().replace(" ", "")
    }
}

#[derive(Debug, FromRow)]
pub struct JsonWebKey {
    /// Unique identifier for each web key
    pub id: String,

    /// The public part of the web key
    pub public_key: String,

    /// The private part of the web key
    pub private_key: String,

    /// Timestamp of when the web key was created
    pub created_at: DateTime<Utc>,
}
