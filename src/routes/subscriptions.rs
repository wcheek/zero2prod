use actix_web::{HttpResponse, Responder, post, web};
use chrono::Utc;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct EmailSubscriptionFormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(
    form: web::Form<EmailSubscriptionFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
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
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
