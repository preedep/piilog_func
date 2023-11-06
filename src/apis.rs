use std::fmt::Display;
use std::time::Duration;

use actix_web::{HttpRequest, web};
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use base64::Engine;
use base64::engine::general_purpose;
use kafka::client::{KafkaClient, ProduceMessage, RequiredAcks, SecurityConfig};
use logs::{debug, error};
use openssl::pkcs12::Pkcs12;
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
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_cipher_list("DEFAULT").unwrap();
    builder.set_verify(SslVerifyMode::PEER);

    let pfx_data = data_cert.value.clone();
    let res_decode = match general_purpose::STANDARD.decode(pfx_data) {
        Ok(c) => {
            let pkcs12 = Pkcs12::from_der(&c).unwrap().parse2("");
            match pkcs12 {
                Ok(pk) => {
                    if let (Some(pk), Some(cert)) = (pk.pkey, pk.cert) {
                        debug!("Set private key and certificate");
                        builder.set_private_key(pk.as_ref()).unwrap();
                        builder.set_certificate(cert.as_ref()).unwrap();
                        builder.check_private_key().expect("Private key failed");
                    }
                    Ok(())
                }
                Err(e) => {
                    //error!("Error get pkcs12: {}", e);
                    Err(PiiLogFuncError::new(e.to_string()))
                }
            }
        }
        Err(e) => {
            //error!("Error decoding certificate : {}", e);
            Err(PiiLogFuncError::new(e.to_string()))
        }
    };

    /*
    builder
        .set_certificate_file(
            "/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl2/kafka.preedee.space.signed.crt.pem",
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_private_key_file(
            "/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl2/kafka.preedee.space.p12.key.pem",
            SslFiletype::PEM,
        )
        .unwrap();

            builder.check_private_key().expect("Private key failed");
    */
    builder
        .set_ca_file("/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl2/rootCA.crt.pem")
        .unwrap();

    match res_decode {
        Ok(_) => {
            let connector = builder.build();
            // ~ instantiate KafkaClient with the previous OpenSSL setup
            let kafka_brokers = data_config
                .kafka_endpoint
                .split(",")
                .map(|c| c.to_string())
                .filter(|c| !c.is_empty())
                .collect::<Vec<String>>();

            let config = SecurityConfig::new(connector).with_hostname_verification(false);
            debug!(
                "List Kafka brokers : {:?} with config : {:#?} ",
                kafka_brokers, config,
            );
            let mut client = KafkaClient::new_secure(kafka_brokers, config);
            match client.load_metadata_all() {
                Ok(_) => {

                    for topic in client.topics().names() {
                        debug!("topic: {}", topic);
                    }

                    let req = vec![ProduceMessage::new(
                        "piiLog",
                        0,
                        None,
                        Some("a|b|a".as_bytes()),
                    )];
                    let resp =
                        client.produce_messages(RequiredAcks::One, Duration::from_millis(100), req);
                    resp.map(|r| {
                        debug!("Response received : {:#?}", r);
                        PiiLogResponse {
                            message: "Sent Completed".to_string(),
                        }
                    })
                        .map_err(|e| {
                            error!("Produce message error : {:?}", e);
                            PiiLogFuncError::new(e.to_string())
                            }
                        )
                }
                Err(e) => {
                    error!("Load Metadata failed : {:?}", e);
                    Err(PiiLogFuncError::new(e.to_string()))
                }
            }
        }
        Err(e) => Err(e),
    }
}
