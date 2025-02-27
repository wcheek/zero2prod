use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct EmailSubscriptionFormData {
    email: String,
    name: String,
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<EmailSubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
