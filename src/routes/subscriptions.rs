use actix_web::{HttpResponse, Responder, post, web};

#[derive(serde::Deserialize)]
struct EmailSubscriptionFormData {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<EmailSubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
