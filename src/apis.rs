use std::fmt::Display;

use actix_web::{HttpRequest, web};
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use base64::Engine;
use base64::engine::general_purpose;
use logs::debug;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use openssl::x509::X509;

use crate::models::{
    PiiLogFuncConfiguration, PiiLogFuncResult, PiiLogRequest, PiiLogResponse,
};

pub async fn post_piilog_func(
    req: HttpRequest,
    data_cert: web::Data<KeyVaultGetSecretResponse>,
    data_config: web::Data<PiiLogFuncConfiguration>,
    payload: web::Json<PiiLogRequest>,
) -> PiiLogFuncResult<PiiLogResponse> {
    debug!("Calling post_piilog_func");

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_cipher_list("DEFAULT").unwrap();
    builder.set_verify(SslVerifyMode::PEER);

    let cert_bytes = general_purpose::STANDARD
        .decode(data_cert.value.as_str()).unwrap();
    println!("{:?}", cert_bytes);

    let x509 = X509::from_pem(&cert_bytes).unwrap();

    builder.set_certificate(&x509).unwrap();
    let connector = builder.build();



    /*
    let access_token = get_azure_access_token(None).await;
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
    }*/

    Ok(PiiLogResponse {
        message: "Sent Completed".to_string(),
    })
}
