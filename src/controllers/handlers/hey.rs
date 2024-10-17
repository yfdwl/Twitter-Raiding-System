use actix_web::{HttpResponse, Responder};
use apistos::api_operation;

#[api_operation(summary = "Get user profile", tag = "user")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
