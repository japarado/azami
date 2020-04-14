use actix_web::middleware::errhandlers::{ ErrorHandlerResponse, ErrorHandlers };
use actix_web::{dev, http, HttpResponse, Result};

pub fn render_401<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
