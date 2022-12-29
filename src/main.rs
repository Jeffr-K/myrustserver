extern crate dotenv;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use dotenv::dotenv;
use std::env;


async  fn health_check(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("hello, world!");
    format!("{}!", &name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
    })
    .bind((
        env::var("SERVER_HOST").unwrap(),
        env::var("SERVER_PORT").unwrap().parse::<u16>().unwrap())
    )?
    .run()
    .await
}
