use axum::routing::get;
use axum::Router;
use reporting::api::repository::Repository;
use reporting::api::{handlers, Env};
use reporting::settings;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = settings::Settings::new().expect("settings parsing failed");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database.url.as_str())
        .await
        .expect("Cannot connect to postgres");

    let repository = Repository::new(pool);
    let env = Arc::new(Env { repository });

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/id/:id", get(handlers::report))
        .route("/datasources", get(handlers::get_datasources))
        .route("/query", get(handlers::query))
        .with_state(env);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to bind to port 3000");
}
