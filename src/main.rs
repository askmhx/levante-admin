extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;

pub mod database;
pub mod util;
pub mod controller;
pub mod repository;
pub mod app;

fn main() {
    let path = std::env::args().nth(1).expect("Missing configration file path");
    let app = app::bootstrap::Bootstrap::new(path);
    app.launch();
}