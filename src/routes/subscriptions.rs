use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    log::info!(
        "[{}] Adding '{}' '{}' as a new subscriber",
        request_id,
        form.name,
        form.email
    );
    log::info!(
        "[{}] Saving new subscriber details in the database",
        request_id
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
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("[{}] New subscriber details have been saved", request_id);
            HttpResponse::Ok()
        }
        Err(e) => {
            log::error!("[{}] Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError()
        }
    }
}
