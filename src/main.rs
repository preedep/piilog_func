use std::env;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use azure_core::auth::TokenCredential;
use logs::debug;

use crate::apis::post_pii_log_func;
use crate::azure_utils::get_certificate_from_key_vault;
use crate::models::PiiLogFuncConfiguration;

mod apis;
mod azure_utils;
mod models;

const AZURE_FUNCTION_PORT: &str = "FUNCTIONS_CUSTOMHANDLER_PORT";
const PII_LOG_ENDPOINT: &str = "PII_LOG_ENDPOINT";
const PII_LOG_KEY_VAULT_ACCOUNT: &str = "PII_LOG_KEY_VAULT_ACCOUNT";
const PII_LOG_KEY_VAULT_KEY_NAME: &str = "PII_LOG_KEY_VAULT_KEY_NAME";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let port_key = AZURE_FUNCTION_PORT;
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 7071,
    };

    //  Required system variables
    let pii_log_endpoint = env::var(PII_LOG_ENDPOINT).expect("PII_LOG_ENDPOINT Invalid");
    let pii_log_key_vault_account =
        env::var(PII_LOG_KEY_VAULT_ACCOUNT).expect("PII_LOG_KEY_VAULT_ACCOUNT Invalid");
    let pii_log_key_vault_key_name =
        env::var(PII_LOG_KEY_VAULT_KEY_NAME).expect("PII_LOG_KEY_VAULT_KEY_NAME Invalid");

    let config = PiiLogFuncConfiguration {
        kafka_endpoint: pii_log_endpoint.clone(),
        key_vault_account: pii_log_key_vault_account.clone(),
        key_vault_key_name: pii_log_key_vault_key_name.clone(),
    };
    debug!("Configuring value : {:#?}", config);
    //
    // Get Azure Credentials
    //
    let res_cert = get_certificate_from_key_vault(
        config.key_vault_account.as_str(),
        config.key_vault_key_name.as_str(),
    )
    .await;
    match res_cert {
        Ok(res_cert) => {
            debug!("Get Key Vault Value : {:#?}", res_cert);
            let data_cert = Data::new(res_cert);
            HttpServer::new(move || {
                App::new()
                    .wrap(middleware::DefaultHeaders::new().add(("PIILog-X-Version", "1.0")))
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .app_data(data_cert.clone())
                    .app_data(Data::new(config.clone()))
                    .service(
                        // prefixes all resources and routes attached to it...
                        web::scope("/api")
                            // ...so this handles requests for `GET /app/index.html`
                            .route("/PiiLogHttpTrigger", web::post().to(post_pii_log_func)),
                    )
            })
            .bind(("0.0.0.0", port))?
            .run()
            .await
        }
        Err(e) => {
            panic!("PiiFunc error: {}", e);
        }
    }
}
