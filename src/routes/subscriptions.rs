use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

use crate::{
    domain::{SubscriberEmail, SubscriberInfo, SubscriberName},
    routes::create_response,
};

#[derive(serde::Deserialize)]
pub struct SubscriberInput {
    pub email: String,
    pub name: String,
}

impl TryFrom<SubscriberInput> for SubscriberInfo {
    type Error = String;
    fn try_from(value: SubscriberInput) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(user_json, pool)
)]
pub async fn insert_subscriber(
    user_json: &SubscriberInfo,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (email, name, subscribed_at)
            VALUES ($1, $2, $3)
        "#,
        user_json.email.as_ref(),
        user_json.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscriber_input, pool),
    fields(
        subscriber_email = %subscriber_input.email,
        subscriber_name= %subscriber_input.name
    )
)]
pub async fn subscribe(
    subscriber_input: web::Json<SubscriberInput>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_subscriber = match subscriber_input.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(e) => return HttpResponse::BadRequest().json(create_response("error", e)),
    };

    match insert_subscriber(&new_subscriber, &pool).await {
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
