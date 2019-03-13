//extern crate actix;
//extern crate diesel;

//use actix_web::{server, App, HttpRequest, Responder};
//use diesel::prelude::*;
//use database::model::*;
//use database::enums::*;

//#[get("/wp-json/wp/v2/posts")]
//pub fn list(req: &HttpRequest) -> impl Responder  {
//    use database::schema::wp_posts::dsl::*;
//    let results = wp_posts
//        .filter(post_status.eq(""))
//        .limit(5)
//        .load::<WpPosts>(&*conn)
//        .expect("Error loading posts");
//    Json(json!({ "status": "ok" }))
//}
//
//#[post("/wp-json/wp/v2/posts", format = "application/json")]
//pub fn create(req: &HttpRequest) -> impl Responder  {
//    Json(json!({ "status": "ok" }))
//}
//
//#[get("/wp-json/wp/v2/posts/<pid>")]
//pub fn retrieve(req: &HttpRequest) -> impl Responder  {
//    Json(json!({ "status": "ok" }))
//}
//
//#[post("/wp-json/wp/v2/posts/<pid>")]
//pub fn update(req: &HttpRequest) -> impl Responder  {
//    Json(json!({ "status": "ok" }))
//}
//
//#[delete("/wp-json/wp/v2/posts/<pid>")]
//pub fn delete(req: &HttpRequest) -> impl Responder  {
//    use database::schema::wp_posts::dsl::*;
//    let result =  diesel::update(wp_posts.filter(ID.eq(pid.parse::<i64>().unwrap()))).set(post_status.eq(POST_STATUS_TYPE_TRASH)).execute(&*conn);
//    Json(json!({ "status": "ok" }))
//}