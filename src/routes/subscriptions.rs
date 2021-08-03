use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Data model for what we need to subscribe a user to our mailing list.
/// We need their email (of course) and their name. Both fields are required.
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// POST endpoint for subscribing to our mailing list
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // run the insert statement into the database
    let result = sqlx::query!(
        // language=SQL
        r#"
        INSERT INTO Subscriptions (Id, Email, Name, SubscribedAt)
        VALUES ($1, $2, $3, $4)        
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;
    // match on the result (200 if successful, 500 if a DB error occurred)
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
