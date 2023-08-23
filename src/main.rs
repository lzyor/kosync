// ╦  ┌─┐┬ ┬┌─┐┬─┐ Lzyor Studio
// ║  ┌─┘└┬┘│ │├┬┘ kosync-project
// ╩═╝└─┘ ┴ └─┘┴└─ https://lzyor.work/koreader/
// 2023 (c) Lzyor

mod api;
mod db;
mod defs;
mod utils;

use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use axum_server::{
    Handle,
    tls_rustls::RustlsConfig,
};
use std::{
    env,
    net::SocketAddr,
    path::PathBuf,
    time::Duration,
};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;

use shadow_rs::shadow;
shadow!(build);

#[tokio::main]
async fn main() {
    // initialize logger
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .pretty()
            .with_line_number(true)
            .with_thread_names(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .compact()
            .without_time()
            .init();
    }

    // config variables
    let config_addr: SocketAddr = env::var("KOSYNC_ADDR")
        .unwrap_or(defs::DEFAULT_ADDR.to_string())
        .parse()
        .expect("[INIT] Failed to parse addr");
    let config_db_path = defs::DEFAULT_DB_PATH;

    // initialize database and router
    let db = db::DB::new(&config_db_path).expect("[INIT] Failed to open database");
    let router = Router::new()
        .route("/users/create", post(api::create_user))
        .merge(
            Router::new()
                .route("/users/auth", get(api::auth_user))
                .route("/syncs/progress", put(api::update_progress))
                .route("/syncs/progress/:doc", get(api::get_progress))
                .route("/healthcheck", get(api::healthcheck))
                .layer(
                    ServiceBuilder::new()
                        .layer(TraceLayer::new_for_http())
                        .layer(middleware::from_fn_with_state(db.clone(), api::auth))
                ),
        )
        .with_state(db);

    // Spawn a task to gracefully shutdown the server
    let handle = Handle::new();
    tokio::spawn(graceful_shutdown(handle.clone()));

    // start server
    if env::var("KOSYNC_NO_TLS").ok().is_some() {
        tracing::info!("[INIT] listening on {} ", config_addr);
        axum_server::bind(config_addr)
            .handle(handle)
            .serve(router.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("[INIT] Failed to start server");
    } else {
        // configure certificate and private key
        let tls_config = RustlsConfig::from_pem_file(
                PathBuf::from(env::var("KOSYNC_CERT")
                    .unwrap_or(defs::DEFAULT_TLS_CERT.to_string())),
                PathBuf::from(env::var("KOSYNC_KEY")
                    .unwrap_or(defs::DEFAULT_TLS_PRIVKEY.to_string())),
            )
            .await
            .expect("[INIT] Failed to parse TLS config");

        tracing::info!("[INIT] listening on {} [TLS]", config_addr);
        axum_server::bind_rustls(config_addr, tls_config)
            .handle(handle)
            .serve(router.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .expect("[INIT] Failed to start server");
    }

    tracing::info!("[EXIT] server is shutting down");
}

// graceful shutdown on SIGINT & SIGTERM
async fn graceful_shutdown(handle: Handle) {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = sigint.recv() => tracing::info!("Caught SIGINT"),
        _ = sigterm.recv() => tracing::info!("Caught SIGTERM"),
    }
    handle.graceful_shutdown(Some(Duration::from_secs(30)));

    // Print a live connection count every second
    loop {
        sleep(Duration::from_secs(1)).await;

        tracing::info!("Live connections: {}", handle.connection_count());
    }
}
