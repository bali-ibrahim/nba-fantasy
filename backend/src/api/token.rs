use log::{error, info};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl, TokenUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenBody {
    grant_type: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

pub async fn token_func_async(
    req: TokenBody,
) -> oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType> {
    let client = BasicClient::new(
        ClientId::new(dotenv!("YAHOO_OAUTH_CLIENT_ID").to_string()),
        Some(ClientSecret::new(
            dotenv!("YAHOO_OAUTH_CLIENT_SECRET").to_string(),
        )),
        AuthUrl::new("http://auth".to_string()).expect("blah"),
        Some(
            TokenUrl::new("https://api.login.yahoo.com/oauth2/get_token".to_string())
                .expect("blah"),
        ),
    )
    .set_redirect_uri(RedirectUrl::new(req.redirect_uri).expect("Issue constructing Redirect url"));

    let pkce_verifier = PkceCodeVerifier::new(req.code_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(req.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            error!("{:?}", err.to_string());
            panic!("TODO better error handling here");
        }
        Ok(val) => {
            info!("Tokens received from OAuth provider!");
            val
        }
    }
}
