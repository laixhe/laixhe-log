mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
    })
    .bind("0.0.0.0:5050")?
    .run()
    .await
}
