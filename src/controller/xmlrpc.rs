mod controller {
    use actix_web::{HttpRequest, Responder};


    fn xmlrpc(req: &HttpRequest) -> impl Responder {
        let to = req.match_info().get("name").unwrap_or("World");
        format!("Hello {}!", to)
    }

}