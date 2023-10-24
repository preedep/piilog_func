use std::env;

use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use azure_core::auth::TokenCredential;
use logs::debug;
use tokio::sync::Mutex;

use crate::apis::post_piilog_func;
use crate::azure_utils::{get_azure_access_token, get_certificate_from_key_vault};

mod apis;
mod azure_utils;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 8088,
    };
    //
    // Get Azure Credentials
    //
    let response = get_azure_access_token(None).await;
    match response {
        Ok(r) => {
            let res_cert =
                get_certificate_from_key_vault("nicksecretstoredev001",
                                               "certkafkadevnick001", &r)
                    .await.expect("Get Certificate from key vault failed ");
            debug!("Get Key Vault Value : {:#?}",res_cert);
            let data_cert = Data::new(res_cert);

            debug!("Response token from azure : {:#?}", r);
            let data_access_token = Data::new(r);
            HttpServer::new(move || {
                App::new()
                    .wrap(middleware::DefaultHeaders::new().add(("PIILog-X-Version", "1.0")))
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .app_data(data_access_token.clone())
                    .app_data(data_cert.clone())
                    .service(
                        // prefixes all resources and routes attached to it...
                        web::scope("/api")
                            // ...so this handles requests for `GET /app/index.html`
                            .route("/PiiLogHttpTrigger", web::post().to(post_piilog_func)),
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
