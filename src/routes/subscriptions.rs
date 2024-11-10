use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

use crate::routes::create_response;

#[derive(serde::Deserialize)]
pub struct UserJSON {
    pub email: String,
    pub name: String,
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(user_json, pool)
)]
pub async fn insert_subscriber(user_json: &UserJSON, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at)
            VALUES ($1, $2, $3)
        "#,
        user_json.email,
        user_json.name,
        Utc::now()
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(user_json, pool),
    fields(
        subscriber_email = %user_json.email,
        subscriber_name= %user_json.name
    )
)]
pub async fn subscribe(user_json: web::Json<UserJSON>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&user_json, &pool).await {
        Ok(_) => HttpResponse::Created()
            .json(create_response("success", "User subscribed successfully.")),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            if let Some(db_error) = e.as_database_error() {
                if db_error.code().as_deref() == Some("23505") {
                    return HttpResponse::Conflict()
                        .json(create_response("error", "Email is already subscribed."));
                };
            }

            return HttpResponse::InternalServerError()
                .json(create_response("error", "Internal Server Error."));
        }
    }
}
