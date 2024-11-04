use reporting::settings::Settings;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = Settings::new().expect("settings parsing failed");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database.url.as_str())
        .await
        .expect("Cannot connect to postgres");
}
