use std::net::TcpListener;
// !lib.rs
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
async fn health_check()-> HttpResponse{
    HttpResponse::Ok().finish()
}

async fn subscribe()-> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
    }).listen(listener)?
        .run();
    Ok(server)
}