use actix_web::web;
use actix_web::web::{HttpResponse};
use actix_web::App;
use actix_files::Files as Fs;

use crate::controller;

pub fn routers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test").route(web::get().to(|| HttpResponse::Ok())));
    cfg.service(Fs::new("/favicon.ico", "/Users/Crazz/WorkSpace/RSWork/levante-admin/statics/").index_file("favicon.ico"));
//    cfg.service(Fs::new("/favicon.ico", "/Users/Crazz/WorkSpace/RSWork/levante-admin/statics/").index_file("index.html"));
//    cfg.service(App::service(controller::xmlrpc));
}