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

use std::{net::SocketAddr, sync::Arc};

use axum::http::header::{self, HeaderValue};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

use crate::{context::Context, extractor, routes, swagger};

pub async fn run(ctx: Arc<Context>) {
    let port = ctx.config.port;

    // Runs database migrations from migrations folder
    if let Err(e) = ctx.database.migrate().await {
        error!("Failed to run database migrations: {}", e);
        std::process::exit(1);
    }

    // Refresh keys from database and update cache
    if let Err(e) = extractor::refresh_keys(ctx.clone()).await {
        error!("Failed to initialize keys: {}", e);
        std::process::exit(1);
    }

    // Build our application with a route
    let Ok(cors) = configure_cors(&ctx.config.allowed_origin) else {
        error!("Invalid CORS configuration: invalid origin format");
        std::process::exit(1);
    };

    let app = routes::build().merge(swagger::build()).layer(cors).with_state(ctx);

    // Run our app with hyper, and serve it over HTTP
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server running on {}", addr);

    // Run this server for ... forever!
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", err);
        std::process::exit(1)
    }
}

/// Configures CORS middleware based on the allowed origin
fn configure_cors(allowed_origin: &Option<Vec<String>>) -> Result<CorsLayer, ()> {
    let layer = CorsLayer::new()
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_methods(Any);

    let Some(origins) = allowed_origin else {
        return Ok(layer);
    };

    if origins.contains(&"*".to_string()) {
        Ok(layer.allow_origin(Any))
    } else {
        let header_values: Vec<HeaderValue> = origins
            .iter()
            .map(|s| s.parse::<HeaderValue>())
            .collect::<Result<_, _>>()
            .map_err(|_| ())?;

        Ok(layer.allow_origin(header_values))
    }
}
