use actix_web::{HttpResponse, Responder, get};

#[get("/health_check")]
async fn check_health() -> impl Responder {
    HttpResponse::Ok().finish()
}
