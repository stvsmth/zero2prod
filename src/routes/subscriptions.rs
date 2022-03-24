use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, conn_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("SAving new subscriber details in the database.");

    tracing::info!(
        "request_id {} - Adding '{}' '{}' as new subscriber.",
        request_id,
        form.email,
        form.name
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(conn_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id - {} New subscriber details saved.", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id - {} Failed to execute query {:?}.",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}