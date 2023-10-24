use actix_web::{HttpRequest, HttpResponseBuilder, Responder, web};
use actix_web::http::StatusCode;
use azure_core::auth::TokenResponse;
use logs::error;
use serde::{Deserialize, Serialize};
use crate::azure_utils::get_azure_access_token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiLogRequest {

}
pub async fn post_piilog_func(req: HttpRequest,
                              access_token: web::Data<TokenResponse>,
                              payload: web::Json<PiiLogRequest>) -> impl Responder {

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
    HttpResponseBuilder::new(StatusCode::OK).finish()
}