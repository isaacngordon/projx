// isaacgordon@Isaacs-Mac-Studio config % pwd
// /Users/isaacgordon/Documents/truesimcha-automation/config
// isaacgordon@Isaacs-Mac-Studio config % ls
// google_service_account.json

use std::{io::{BufRead, BufReader, Write}, net::TcpListener};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use reqwest::Url;
use serde::Deserialize;

const GOOGLE_ACCESS_TOKEN_SAVE_PATH: &str = "./config/access_token";
const GOOGLE_REFRESH_TOKEN_SAVE_PATH: &str = "./config/refresh_token";
const GOOGLE_SERVICE_ACCOUNT_PATH: &str = "./config/google_service_account.json";

#[derive(Deserialize, Debug)]
struct GoogleServiceAccount {
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
    universe_domain: String,
    client_secret: Option<String>,
}

impl Default for GoogleServiceAccount {
    /// Load the service account from the environment variable or the default path
    fn default() -> Self {
        // Load the service account file from the environment variable or the default path
        let filepath = std::env::var("GOOGLE_SERVICE_ACCOUNT_PATH")
            .unwrap_or_else(|_| GOOGLE_SERVICE_ACCOUNT_PATH.to_string());
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").ok();

        // Parse the service account file
        let service_account_file = std::fs::read_to_string(filepath)
            .map_err(|e| crate::error::Error::IoError(e))
            .unwrap();

        let mut a: GoogleServiceAccount = serde_json::from_str(&service_account_file).unwrap();
        a.client_secret = client_secret;
        a
    }
}

impl GoogleServiceAccount {
    /// Create a new GoogleServiceAccount from a file path
    pub fn new(filepath: &str, client_secret: &str) -> crate::error::Result<Self> {
        let service_account_file = std::fs::read_to_string(filepath)?;
        let mut service_account: GoogleServiceAccount =
            serde_json::from_str(&service_account_file)?;
        service_account.client_secret = Some(client_secret.to_string());
        Ok(service_account)
    }
}

pub async fn example_raw_get_access_and_refresh_tokens() -> crate::error::Result<Vec<String>> {
    // Load the service account file
    let service_account = GoogleServiceAccount::default();
    let oauth_client = BasicClient::new(
        ClientId::new(service_account.client_id),
        None, 
        // Some(ClientSecret::new(service_account.client_secret.ok_or(
        //     crate::error::Error::CustomError("No client secret".to_owned()),
        // )?))
        AuthUrl::new(service_account.auth_uri).unwrap(),
        Some(TokenUrl::new(service_account.token_uri).unwrap()),
    ).set_revocation_uri(
        oauth2::RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );;

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = oauth_client
        .authorize_url(oauth2::CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(oauth2::Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!("Open this URL in your browser:\n{authorize_url}\n");

    // // Temp server that will collect the code and state from the redirect URL. Ie its a temp callback server
    // let (code, state) = {
    //     // A very naive implementation of the redirect server.
    //     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    //     // The server will terminate itself after collecting the first code.
    //     let Some(mut stream) = listener.incoming().flatten().next() else {
    //         panic!("listener terminated without accepting a connection");
    //     };

    //     let mut reader = BufReader::new(&stream);

    //     let mut request_line = String::new();
    //     reader.read_line(&mut request_line).unwrap();

    //     let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    //     let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

    //     let code = url
    //         .query_pairs()
    //         .find(|(key, _)| key == "code")
    //         .map(|(_, code)| oauth2::AuthorizationCode::new(code.into_owned()))
    //         .unwrap();

    //     let state = url
    //         .query_pairs()
    //         .find(|(key, _)| key == "state")
    //         .map(|(_, state)| oauth2::CsrfToken::new(state.into_owned()))
    //         .unwrap();

    //     let message = "Go back to your terminal :)";
    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
    //         message.len(),
    //         message
    //     );
    //     stream.write_all(response.as_bytes()).unwrap();

    //     (code, state)
    // };

    // println!("Google returned the following code:\n{}\n", code.secret());
    // println!(
    //     "Google returned the following state:\n{} (expected `{}`)\n",
    //     state.secret(),
    //     csrf_state.secret()
    // );

    // // Exchange the code with a token.
    // let token_response = oauth_client
    //     .exchange_code(code)
    //     .set_pkce_verifier(pkce_code_verifier)
    //     .request(&http_client);

    // println!("Google returned the following token:\n{token_response:?}\n");

    // // Revoke the obtained token
    // let token_response = token_response.unwrap();
    // let token_to_revoke: oauth2::StandardRevocableToken = match token_response.refresh_token() {
    //     Some(token) => token.into(),
    //     None => token_response.access_token().into(),
    // };

    // oauth_client
    //     .revoke_token(token_to_revoke)
    //     .unwrap()
    //     .request(&http_client)
    //     .expect("Failed to revoke token");




    let access_token = "dummy".to_string();
    let refresh_token = "dummy".to_string();

    println!("Access token: {}", access_token);
    println!("Refresh token: {}", refresh_token);

    Ok(vec![access_token, refresh_token])
}

fn save_access_and_refresh_tokens(
    access_token: &str,
    refresh_token: &str,
) -> crate::error::Result<()> {
    // Save the access token to the file system
    let access_token_save_path = format!("{}", GOOGLE_ACCESS_TOKEN_SAVE_PATH);
    println!("Saving access token to {}", access_token_save_path);
    let mut access_token_file = std::fs::File::create(access_token_save_path)?;
    access_token_file.write_all(access_token.as_bytes())?;
    println!("Saved.");

    // Save the refresh token to the file system
    let refresh_token_save_path = format!("{}", GOOGLE_REFRESH_TOKEN_SAVE_PATH);
    println!("Saving refresh token to {}", refresh_token_save_path);
    let mut refresh_token_file = std::fs::File::create(refresh_token_save_path)?;
    refresh_token_file.write_all(refresh_token.as_bytes())?;
    println!("Saved.");

    Ok(())
}

mod test {
    use super::*;

    #[tokio::test]
    async fn test_example_raw_get_and_save_tokens() {
        let tokens = example_raw_get_access_and_refresh_tokens().await.unwrap();
        save_access_and_refresh_tokens(&tokens[0], &tokens[1]).unwrap();
    }
}
