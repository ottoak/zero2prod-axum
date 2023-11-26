use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zero2prod_axum::configuration::Settings;
use zero2prod_axum::startup::run;
use zero2prod_axum::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = Settings::get_config().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(addr).expect("Failed to bind port");

    run(listener, connection_pool)?.await
}
