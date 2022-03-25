use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(form, conn_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: web::Form<FormData>, conn_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    match insert_subscriber(&conn_pool, &form).await {
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

#[tracing::instrument(name = "Saving new subscriber to the database", skip(form, conn_pool))]
pub async fn insert_subscriber(conn_pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(conn_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}.", e);
        e
    })?;
    Ok(())
}
