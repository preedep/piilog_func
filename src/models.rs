use std::fmt::{Display, Formatter};

use actix_web::{error, HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogFuncConfiguration {
    #[serde(rename = "kafka_endpoint")]
    pub kafka_endpoint: String,
    #[serde(rename = "key_vault_account")]
    pub key_vault_account: String,
    #[serde(rename = "key_vault_key_name")]
    pub key_vault_key_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogRequest {
    #[serde(rename = "app_id")]
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogResponse {
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogFuncError {
    #[serde(rename = "error_message")]
    pub message: String,
}

pub type PiiLogFuncResult<T> = Result<T, PiiLogFuncError>;

impl Display for PiiLogFuncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl PiiLogFuncError {
    pub(crate) fn new(message: String) -> Self {
        PiiLogFuncError { message }
    }
}

impl error::ResponseError for PiiLogFuncError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        type BoxBody = PiiLogFuncError;
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl Responder for PiiLogResponse {
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();
        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}
