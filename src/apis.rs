use std::fmt::{Display, Formatter};
use actix_web::{error, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use azure_core::auth::TokenResponse;
use logs::error;
use serde::{Deserialize, Serialize};
use crate::azure_utils::get_azure_access_token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogRequest {

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogResponse {

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogFuncError {
    #[serde(rename = "error_message")]
    pub message: String,
}


type PiiLogFuncResult<T> = Result<T,PiiLogFuncError>;

impl Display for PiiLogFuncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:#?}",self)
    }
}

impl error::ResponseError for PiiLogFuncError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        type BoxBody = PiiLogFuncError;
        HttpResponse::build(self.status_code())
            .json(self)
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

pub async fn post_piilog_func(req: HttpRequest,
                              access_token: web::Data<TokenResponse>,
                              payload: web::Json<PiiLogRequest>) -> PiiLogFuncResult<PiiLogResponse> {

    let access_token = access_token.get_ref();
    let access_token = get_azure_access_token(Some(access_token.clone())).await;
    match access_token {
        Ok(a) => {
           let _ = req.app_data().insert(&a);

        }
        Err(e) => {
            error!("Error posting request to API: {}", e);
        }
    }
    Ok(PiiLogResponse{

    })
}