use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

use crate::routes::create_response;

#[derive(serde::Deserialize)]
pub struct UserJSON {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(user_json: web::Json<UserJSON>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = uuid::Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        user_json.email,
        user_json.name
    );

    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at)
            VALUES ($1, $2, $3)
        "#,
        user_json.email,
        user_json.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Created()
                .json(create_response("success", "User subscribed successfully."))
        }
        Err(e) => {
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
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
