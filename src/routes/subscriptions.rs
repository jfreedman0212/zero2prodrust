use actix_web::{web, HttpResponse};

/// Data model for what we need to subscribe a user to our mailing list.
/// We need their email (of course) and their name. Both fields are required.
#[derive(serde::Deserialize)]
#[allow(dead_code)] // FIXME: remove this once we actually implement subscribe call!
pub struct FormData {
    email: String,
    name: String,
}

/// POST endpoint for subscribing to our mailing list
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
