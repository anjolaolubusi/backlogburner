use actix_web::{App, HttpServer};
use actix_cors::Cors;
mod endpoints;
mod model;
mod ga;
mod aco;
extern crate chrono;
extern crate rsgenetic;

/**
 * Connects GACO to the HTTP Endpoint and runs the web API
 * Pre-condition: None
 * Post-condition: Hobby events are sent to the user
 */
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let url = match cfg!(debug_assertions){
        true => "127.0.0.1",
        false => "backlogburner.com"
    };
    println!("Hosting URL: {}", url);
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(endpoints::echo)
            .service(model::getNewSchedule)
    })
    .bind((url, 5000))?
    .run()
    .await
}
