use actix_web::{HttpResponse, Responder};
use apistos::api_operation;

#[api_operation(summary = "Get Raids Points", tag = "user")]
pub async fn get_raids_points() -> impl Responder {
    HttpResponse::Ok().body("Raids Points")
}
