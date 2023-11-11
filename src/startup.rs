use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;

type App = Server<AddrIncoming, IntoMakeService<Router>>;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    // build our application
    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
