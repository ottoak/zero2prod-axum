use std::net::TcpListener;

use axum::{
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;

use crate::routes::{health_check, home, subscribe};

type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    // build our application
    let app = Router::new()
        .route("/", get(home))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
