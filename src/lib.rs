use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
// We were returning `impl Responder` at the very beginning.
// We are now spelling out the type explicitly given that we have
// become more familiar with `actix-web`.
// There is no performance difference! Just a stylistic choice :)
async fn health_check() -> HttpResponse {
HttpResponse::Ok().finish()
}




pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
let server = HttpServer::new(|| {
App::new()
.route("/health_check", web::get().to(health_check))
// A new entry in our routing table for POST /subscriptions requests
.route("/subscriptions", web::post().to(subscribe))
})
.listen(listener)?
.run();
Ok(server)
}

#[derive(serde::Deserialize)]
struct FormData {
email: String,
name: String
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
HttpResponse::Ok().finish()
}
