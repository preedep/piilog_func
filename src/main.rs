use std::env;
use actix_web::{App, HttpServer, web};
use crate::apis::post_piilog_func;

mod apis;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| {
        App::new().service(
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