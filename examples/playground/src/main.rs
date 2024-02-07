use axum::{response::IntoResponse, routing::get, Router};
use playground::{routes, state::WebHtmxState};
use std::{net::SocketAddr, sync::Arc};

mod playground;

#[tokio::main]
async fn main() {
    let web_htmx_state = WebHtmxState {
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
    };

    let app = Router::new()
        .nest("/playground", routes(web_htmx_state))
        .route("/healthcheck", get(get_health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_health_check() -> impl IntoResponse {
    "OK"
}
