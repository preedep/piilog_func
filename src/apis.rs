use std::fmt::Display;
use std::time::Duration;

use actix_web::{HttpRequest, web};
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use kafka::client::{KafkaClient, ProduceMessage, RequiredAcks, SecurityConfig};
use logs::{debug, error};
use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};

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
    builder
        .set_certificate_file(
            "/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl/server.pem",
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_private_key_file(
            "/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl/server.key.pem",
            SslFiletype::PEM,
        )
        .unwrap();
    builder.check_private_key().expect("Private key failed");


    builder
        .set_ca_file("/Users/preedee/Project/Kafka/kafka_2.12-3.5.1/ssl/ca.pem")
        .unwrap();

    //builder.set_ca_file("/Users/preedee/Project/kafka/kafka-docker/certs/kafka.keystore.jks.crt").unwrap();
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
        SecurityConfig::new(connector).with_hostname_verification(false),
    );
    match client.load_metadata_all() {
        Ok(_) => {
            let req = vec![
                ProduceMessage::new("piilog", 0, None, Some("a|b|a".as_bytes())),
                ProduceMessage::new("piilog", 0, None, Some("b|b|b".as_bytes())),
            ];
            let resp = client.produce_messages(RequiredAcks::One, Duration::from_millis(100), req);

            debug!("Response from Kafka broker: {:#?}", resp);
            Ok(PiiLogResponse {
                message: "Sent Completed".to_string(),
            })
        }
        Err(e) => {
            error!("Load Metadata failed : {:?}", e);
            Err(PiiLogFuncError::new(e.to_string()))
        }
    }
}
