use std::fmt::Display;

use actix_web::{HttpRequest, web};
use azure_core::auth::TokenResponse;
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use logs::{debug, error};

use crate::azure_utils::get_azure_access_token;
use crate::models::{PiiLogFuncError, PiiLogFuncResult, PiiLogRequest, PiiLogResponse};

pub async fn post_piilog_func(req: HttpRequest,
                              data_access_token: web::Data<TokenResponse>,
                              data_cert: web::Data<KeyVaultGetSecretResponse>,
                              payload: web::Json<PiiLogRequest>) -> PiiLogFuncResult<PiiLogResponse> {
    debug!("Calling post_piilog_func");

    let access_token = data_access_token.get_ref();
    let access_token = get_azure_access_token(Some(access_token.clone())).await;
    match access_token {
        Ok(a) => {
            //let _ = req.app_data().insert(&a);
            Ok(PiiLogResponse {
                message: "Sent Completed".to_string(),
            })
        }
        Err(e) => {
            error!("Error posting request to API: {}", e);
            Err(PiiLogFuncError::new(e.to_string()))
        }
    }
}