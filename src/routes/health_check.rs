use actix_web::HttpResponse;

/// Health Check Endpoint: Always returns a 200 status code
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
