use r_tracing::init_logging;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // log server and level
    let server = std::env::var("LOG_SERVER").unwrap_or("anita::api".to_string());
    let level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());

    // api port and database url
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let guard = init_logging(server, level);
    let _api = r_api::init_api(port, &database_url).await.expect("could not start api server");
    drop(guard);
}
