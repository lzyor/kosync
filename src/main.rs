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
use std::{env, net::SocketAddr};

use shadow_rs::shadow;
shadow!(build);

#[tokio::main]
async fn main() {
    // initialize logger
    if cfg!(release) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .compact()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .pretty()
            .with_line_number(true)
            .with_thread_names(true)
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
                .layer(middleware::from_fn_with_state(db.clone(), api::auth)),
        )
        .with_state(db);

    // start server
    tracing::info!("[INIT] listening on {}", config_addr);
    axum::Server::bind(&config_addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .expect("[INIT] Failed to start server");

    // graceful shutdown on SIGINT
    tracing::info!("[EXIT] server is shutting down");
}
