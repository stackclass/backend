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

use crate::errors::Result;
use sqlx::{pool::PoolOptions, Pool, Postgres};

pub type Transaction<'a> = sqlx::Transaction<'a, Postgres>;

/// Database connection pool wrapper for PostgreSQL
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    /// Creates new database connection pool
    pub async fn new(database_url: &str) -> Result<Self> {
        Ok(Self {
            pool: PoolOptions::new()
                .test_before_acquire(true)
                .max_connections(50)
                .connect(database_url)
                .await?,
        })
    }

    /// Returns reference to the connection pool
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Runs database migrations from migrations folder
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
}
