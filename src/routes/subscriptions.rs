use axum::{Extension, Form};
use chrono::Utc;
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<FormData>,
) -> StatusCode {
    // Generate a Uuid to tag all logs from one request with a Request Id
    let request_id = Uuid::new_v4();
    //Spans, like logs, have an associated level. 'Info_span' creates a span at the 'Info' log level
    let request_span = tracing::info_span!("Adding a new subscriber", %request_id, subscribe_email = %form.email, subscriber_name = %form.name);
    let _request_span_guard = request_span.enter();
    //'request_span_guard' is dropped at the end of the 'subscribe' function; that is when we 'exit' the span
    // We do not call enter() on query_span; the instrumen struct takes care of it in the query Future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
    let value = sqlx::query!(
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
    .execute(&pool)
    //First, we attach the instrumentation, then we 'await' it
    .instrument(query_span)
    .await;
    match value {
        Ok(_) => {
            tracing::info!("Request_id {} - New subscriber details have been saved. :-)", request_id);
            StatusCode::OK
        },
        Err(e) => {
            tracing::error!("Request_id {} - Failed to execute query: {:?}", request_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
