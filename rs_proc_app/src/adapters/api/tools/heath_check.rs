use rocket::{Build, Rocket};
use rocket::serde::json::serde_json::json;
use rs_commons::domains::http_entities::api_response::ApiResponse;

#[inline]
pub fn routes(rocket_build: Rocket<Build>) -> Rocket<Build> {
    rocket_build.mount("/tools", routes![
        health_check
    ])
}

#[get("/health_check")]
pub async fn health_check() -> ApiResponse {
    ApiResponse::app_api_success(Some(json!({"i'm": "OK"})))
}