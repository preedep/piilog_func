use actix_web::{HttpResponseBuilder, Responder, web};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogRequest {}

pub async fn post_piilog_func(payload: web::Json<PiiLogRequest>) -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK).finish()
}