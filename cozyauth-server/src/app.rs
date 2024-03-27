// © Copyright 2024 Jan Ehrhardt
// SPDX-License-Identifier: AGPL-3.0-or-later OR Apache-2.0

use crate::api::health;
use axum::Router;
use tokio::signal;

pub fn app() -> Router {
    Router::new().merge(health::router())
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
