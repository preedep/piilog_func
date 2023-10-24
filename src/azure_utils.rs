use actix_web::web::Data;
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_identity::DefaultAzureCredential;
use logs::debug;
use time::OffsetDateTime;

const TOKEN_ENDPOINT: &'static str = "https://management.azure.com";

fn now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
pub async fn get_azure_access_token(access_token: Option<TokenResponse>) -> azure_core::Result<TokenResponse>{
    match access_token {
        None => {
            debug!("Last Access Token is empty");
            let credential = DefaultAzureCredential::default();
            credential
                .get_token(TOKEN_ENDPOINT)
                .await
        }
        Some(token) => {
            debug!("Access Token is {:#?}", token);
            if token.expires_on >= now() {
                debug!("Access token expired , try to refresh");
                let credential = DefaultAzureCredential::default();
                credential
                    .get_token(TOKEN_ENDPOINT)
                    .await
            }else{
                Ok(token)
            }
        }
    }
}