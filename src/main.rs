pub mod models;



use actix_web::{web, App, HttpServer, Responder};



#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().service(
            
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
