use axum::{Extension, Form};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subsriber_name = %form.name,
    )
)]

//fn subscribe now orchestras work to be done by translating outcomes according to the chosen web framework's data types and functions (i.e. Axum)
pub async fn subscribe(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<FormData>,
) -> StatusCode {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details into the database."
    skip(form, pool)
)]

//fn insert_subscriber take care of the database logic without any awareness of a specific web framework's functions
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    let _ = sqlx::query!(
        r#"
        INSERT into subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    //We use 'get_ref' to get an immutable reference to 'PgConnection' type wrapped in an Arc smart pointer
    .execute(pool)
    .await;

    Ok(())
}