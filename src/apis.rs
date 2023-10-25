use std::fmt::Display;
use std::time::Duration;

use actix_web::{HttpRequest, web};
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use base64::Engine;
use base64::engine::general_purpose;
use kafka::client::{KafkaClient, ProduceMessage, RequiredAcks, SecurityConfig};
use logs::{debug, error};
use openssl::error::ErrorStack;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::{PKey, Private};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};

use crate::models::{
    PiiLogFuncConfiguration, PiiLogFuncError, PiiLogFuncResult, PiiLogRequest, PiiLogResponse,
};

pub async fn post_pii_log_func(
    _req: HttpRequest,
    data_cert: web::Data<KeyVaultGetSecretResponse>,
    data_config: web::Data<PiiLogFuncConfiguration>,
    payload: web::Json<PiiLogRequest>,
) -> PiiLogFuncResult<PiiLogResponse> {
    debug!("Calling post_piilog_func");

    let cert_value = data_cert.value.clone();

    /*
    let end_private_key = "\n-----END PRIVATE KEY-----\n";
    let start_certificate_key = "-----BEGIN CERTIFICATE-----\n";

    let idx_end_private_key = cert_value.clone().find(end_private_key);
    let private_key = match idx_end_private_key {
        None => "",
        Some(s) => {
            let end = s + end_private_key.len();
            match cert_value.get(0..end) {
                None => "",
                Some(c) => c,
            }
        }
    };
    let certificate_key = match cert_value.clone().find(start_certificate_key) {
        None => "",
        Some(s) => {
            let start = s;
            match cert_value.get(start..) {
                None => "",
                Some(c) => c,
            }
        }
    };
    debug!("Private key \r\n{}", private_key);
    debug!("Certificate key \r\n{}", certificate_key);
    */

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_cipher_list("DEFAULT").unwrap();
    builder.set_verify(SslVerifyMode::PEER);

    let cert_bytes = general_purpose::STANDARD
        .decode(data_cert.value.as_str()).unwrap();

    let pkcs12 = Pkcs12::from_der(&cert_bytes).unwrap().parse2("");
    match pkcs12 {
        Ok(pk) => {
            match pk.cert {
                None => {
                    error!("Don't have cert");
                }
                Some(x509) => {
                    debug!("Have cert : {:#?}",x509);
                    builder.set_certificate(&x509).unwrap();
                }
            }
            match pk.pkey {
                None => {
                    error!("Don't have private key");
                }
                Some(pk) => {
                    debug!("Private key : {:#?}",pk);
                    builder.set_private_key(&pk).unwrap();
                }
            }
        }
        Err(e) => {
            error!("Parse PKCS12 failed : {}",e);
        }
    }
    let chk_pk = builder.check_private_key();
    match chk_pk {
        Ok(_) => {}
        Err(e) => {
            error!("Check private key failed : {}",e);
        }
    }

    let connector = builder.build();

    // ~ instantiate KafkaClient with the previous OpenSSL setup
    let kafka_brokers = data_config
        .kafka_endpoint
        .split(",")
        .map(|c| c.to_string())
        .filter(|c| !c.is_empty())
        .collect::<Vec<String>>();

    debug!("List Kafka brokers : {:?}", kafka_brokers);
    let mut client = KafkaClient::new_secure(
        kafka_brokers,
        SecurityConfig::new(connector).with_hostname_verification(true),
    );
    match client.load_metadata_all() {
        Ok(_) => {
            let req = vec![
                ProduceMessage::new("piilog", 0, None, Some("a".as_bytes())),
                ProduceMessage::new("piilog", 0, None, Some("b".as_bytes())),
            ];
            let resp = client.produce_messages(RequiredAcks::One,
                                               Duration::from_millis(100), req);

            debug!("Response from Kafka broker: {:#?}", resp);
            Ok(PiiLogResponse {
                message: "Sent Completed".to_string(),
            })
        }
        Err(e) => {
            error!("Kafka client error : {:?}", e);
            Err(PiiLogFuncError::new(e.to_string()))
        }
    }
}
