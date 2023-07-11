use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HealthCheck {
    status: bool,
    message: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    let data: HealthCheck = serde_json::from_str(r#"{"status": true, "message": "Ok"}"#).unwrap();
    HttpResponse::Ok().json(data)
}
