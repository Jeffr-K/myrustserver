use actix_web::{App, HttpServer, Responder, web};

async  fn health_check() -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("hello, world!");
    format!("Hello {}!", &name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .router("/", web::get().to(health_check))
            .router()
    })
    .bind(("127.0.0.1:8000"))?
    .run()
    .await
}
