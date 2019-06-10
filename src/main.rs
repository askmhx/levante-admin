#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub mod app;
pub mod controller;
pub mod database;
pub mod model;
pub mod repository;
pub mod util;

fn main() {
    let path = std::env::args().nth(1).expect("Missing configration file path");
    let app = app::bootstrap::Bootstrap::new(path);
    app.launch();
}