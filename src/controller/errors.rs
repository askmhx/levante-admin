//use actix_web::error;
//use actix_web::HttpResponse;

//#[derive(Fail, Debug)]
//enum UserError {
//    #[fail(display="An internal error occurred. Please try again later.")]
//    InternalError,
//}
//
//impl error::ResponseError for UserError {
//    fn error_response(&self) -> HttpResponse {
//        match *self {
//            UserError::InternalError => HttpResponse::new(
//                http::StatusCode::INTERNAL_SERVER_ERROR),
//        }
//    }
//}
