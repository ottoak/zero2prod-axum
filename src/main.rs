use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod_axum::configuration::Settings;
use zero2prod_axum::startup::run;
use zero2prod_axum::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = Settings::get_config().expect("Failed to read configuration.");

    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let addr = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(addr).expect("Failed to bind port");

    run(listener, connection_pool)?.await
}
