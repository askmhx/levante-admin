extern crate actix;
extern crate diesel;

use actix_web::{HttpRequest, Responder};


fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}
