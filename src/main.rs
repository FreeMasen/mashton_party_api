extern crate actix_web;
extern crate postgres;
use actix_web::{http, server, Path, App, Responder};


fn main() {
    server::new(|| App::new().route("/api/parties", http::Method::GET, parties))
    .bind("0.0.0.0:8888").expect("Unable to bind server")
    .run();
}

fn parties(_: Path<()>) -> impl Responder {
    format!("[]")
}