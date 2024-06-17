use crate::errors::SrvError;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Handler to get the liveness of the service
#[actix_web::get("/health")]
pub async fn get_health() -> actix_web::Result<impl Responder, SrvError> {
    let response = HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    });
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_health() {
        let app = App::new().service(get_health);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_health_response() -> Result<(), anyhow::Error> {
        let client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Priority", "u=1, i".parse().unwrap());
        headers.insert(
            "Referer",
            "https://www.dextools.io/app/cn/solana/pairs"
                .parse()
                .unwrap(),
        );
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".parse().unwrap());

        let request = client.request(reqwest::Method::GET, "https://www.dextools.io/shared/analytics/pairs?limit=51&interval=24h&page=1&chain=solana&exchange=dexut1mp8&minLiquidity=1000000&dextScore=80&excludeNative=true")
        .headers(headers);

        let response = request.send().await?;
        let body = response.text().await?;

        println!("{}", body);
        Ok(())
    }
}
