use actix_web::{HttpRequest,post};

#[post("/xmlrpc.php")]
fn xmlrpc(req: HttpRequest) -> String {
    format!("Hello {}! id:{}", req.uri(), req.method())
}