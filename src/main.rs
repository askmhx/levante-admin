#[macro_use]
extern crate diesel;

pub mod app;
pub mod controller;
pub mod database;
pub mod model;
pub mod repository;
pub mod util;

use std::io;
use actix_web::{App, HttpServer, middleware};
use crate::database::manager;
use crate::app::route::routers;
use crate::app::config::AppConfig;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=info");
    let filepath = std::env::args().nth(1).expect("Missing configration file path");
    let config = AppConfig::from_file(filepath).unwrap();
    let pool = manager::init_pool(config.database.to_string());
    let sys = actix_rt::System::new("levante-admin");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routers)
            .wrap(middleware::Logger::default())
    }).bind(config.server.to_string())?.start();
    sys.run()
}
