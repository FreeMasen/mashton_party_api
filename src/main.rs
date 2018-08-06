extern crate actix_web;
extern crate env_logger;
extern crate uuid;
extern crate data;

use actix_web::{
    App,
    Either,
    http,
    HttpRequest,
    HttpResponse,
    Json,
    middleware::{
        Logger,
    },
    Path,
    Responder,
    server,
};
use uuid::Uuid;

use data::Rsvp;

fn main() {
    env_logger::init();
    server::new(|| App::new()
                .middleware(Logger::default())
                .middleware(Logger::new("%a %{User-Agent}i"))
                .route("/api/parties", http::Method::GET, parties)
                .resource("/api/user/invite/{id}", |r| r.method(http::Method::GET).with(user_invite))
                .resource("/api/rsvp", |r| r.method(http::Method::POST).with(update_rsvp))
                .resource("/api/rsvps/{token}",|r| r.method(http::Method::GET).with(rsvps))
    )
    .bind("0.0.0.0:8888").expect("Unable to bind server")
    .run();
}

fn parties(_: HttpRequest) -> impl Responder {
    println!("GET /api/parties");
    if let Some(parties) = data::get_all_parties() {
        Either::A(Json(parties))
    } else {
        Either::B(five_hundred("Unable to get parties"))
    }
}

fn rsvps(path: Path<Uuid>) -> impl Responder {
    if let Some(rsvps) = data::get_rsvps_for(&path.into_inner()) {
        Either::A(Json(rsvps))
    } else {
        Either::B(five_hundred("Unable to get rsvps"))
    }
}
fn user_invite(path: Path<Uuid>) -> impl Responder {
    if let Some(user) = data::get_user_for(&path.into_inner()) {
        Either::A(Json(user))
    } else {
        Either::B(five_hundred("Unable to get user info"))
    }
}

fn update_rsvp(body: Json<Rsvp>) -> impl Responder {
    if let Some(parties) = data::update_rsvp(&body.into_inner()) {
        Either::A(Json(parties))
    } else {
        Either::B(five_hundred("Unable to update rsvp"))
    }
}


fn five_hundred(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError().body(msg.to_string())
}