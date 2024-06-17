#[actix_web::main]
async fn main() {
    dotenvy::dotenv().ok();
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _ = r_api::init_api(port, &database_url).await;
}
