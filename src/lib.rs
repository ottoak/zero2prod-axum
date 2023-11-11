use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use std::net::TcpListener;
type App = Server<AddrIncoming, IntoMakeService<Router>>;
async fn root() -> &'static str {
    "Hello, World!"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> hyper::Result<App> {
    // build our application
    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check));

    // run it with hyper on localhost:8000
    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
