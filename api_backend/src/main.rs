use actix_web::{App, HttpServer};
use actix_cors::Cors;
mod endpoints;
mod model;
extern crate chrono;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(endpoints::echo)
            .service(model::getNewSchedule)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
