use actix_web::{get, post, HttpResponse, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{:?}", req_body);
    HttpResponse::Ok().body(req_body)
}
