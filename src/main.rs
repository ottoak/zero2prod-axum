use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod_axum::configuration::Settings;
use zero2prod_axum::startup::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = Settings::get_config().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let addr = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(addr).expect("Failed to bind port");

    run(listener, connection_pool)?.await
}
