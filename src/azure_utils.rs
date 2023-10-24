use std::sync::Arc;

use azure_core::auth::{TokenCredential, TokenResponse};
use azure_identity::{DefaultAzureCredential, DefaultAzureCredentialBuilder};
use azure_security_keyvault::prelude::KeyVaultGetSecretResponse;
use azure_security_keyvault::SecretClient;
use logs::{debug, error};
use time::OffsetDateTime;

const TOKEN_ENDPOINT: &'static str = "https://management.azure.com";

fn now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub async fn get_azure_access_token(access_token: Option<TokenResponse>) -> azure_core::Result<TokenResponse> {
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
            } else {
                Ok(token)
            }
        }
    }
}
pub async fn get_certificate_from_key_vault(account_name: &str, key_name: &str) -> azure_core::Result<KeyVaultGetSecretResponse> {
    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .build(),
    );
    let key_vault_url = format!("https://{}.vault.azure.net", account_name);
    debug!("Key vault url is {:#?}",key_vault_url);

    let client_res = SecretClient::new(&key_vault_url, creds);
    let res = match client_res {
        Ok(client) => {
            client.get(key_name).await
        }
        Err(e) => {
            error!("Error getting key vault response : {}",e);
            Err(e)
        }
    };
    res
}