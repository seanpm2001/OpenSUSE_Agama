use agama_lib::{auth::AuthToken, error::ServiceError};
use clap::Subcommand;

use crate::error::CliError;
use agama_lib::base_http_client::BaseHTTPClient;
use inquire::Password;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::io::{self, IsTerminal};

/// HTTP Client for auth queries
struct AuthHTTPClient {
    api: BaseHTTPClient,
}

impl AuthHTTPClient {
    pub fn load(client: BaseHTTPClient) -> Result<Self, ServiceError> {
        Ok(Self { api: client })
    }

    /// Necessary http request header for authenticate
    fn authenticate_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers
    }

    /// Query web server for JWT
    ///
    /// TODO:
    /// for now it doesn't use BaseHTTPClient's post and similar methods as it needs
    /// to update query headers
    pub async fn get_jwt(&self, password: String) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let response = client
            // TODO: BaseHTTPClient::url is private, so duplicate it for now.
            .post(self.api.base_url.clone() + "/auth")
            .headers(Self::authenticate_headers())
            .body(format!("{{\"password\": \"{}\"}}", password))
            .send()
            .await?;
        let body = response
            .json::<std::collections::HashMap<String, String>>()
            .await?;
        let value = body.get("token");

        if let Some(token) = value {
            return Ok(token.clone());
        }

        Err(anyhow::anyhow!("Failed to get authentication token"))
    }
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    /// Authenticate with Agama's server and store the token.
    ///
    /// This command tries to get the password from the standard input. If it is not there, it asks
    /// the user interactively. Upon successful login, it stores the token in .agama/agama-jwt. The
    /// token will be automatically sent to authenticate the following requests.
    Login,
    /// Deauthenticate by removing the token.
    Logout,
    /// Print the used token to the standard output.
    Show,
}

/// Main entry point called from agama CLI main loop
pub async fn run(client: BaseHTTPClient, subcommand: AuthCommands) -> anyhow::Result<()> {
    let auth_client = AuthHTTPClient::load(client)?;

    match subcommand {
        AuthCommands::Login => login(auth_client, read_password()?).await,
        AuthCommands::Logout => logout(),
        AuthCommands::Show => show(),
    }
}

/// Reads the password
///
/// It reads the password from stdin if available; otherwise, it asks the
/// user.
fn read_password() -> Result<String, CliError> {
    let stdin = io::stdin();
    let password = if stdin.is_terminal() {
        ask_password()?
    } else {
        let mut buffer = String::new();
        stdin
            .read_line(&mut buffer)
            .map_err(CliError::StdinPassword)?;
        buffer
    };
    Ok(password)
}

/// Asks interactively for the password. (For authentication, not for changing it)
fn ask_password() -> Result<String, CliError> {
    Password::new("Please enter the root password:")
        .without_confirmation()
        .prompt()
        .map_err(CliError::InteractivePassword)
}

/// Logs into the installation web server and stores JWT for later use.
async fn login(client: AuthHTTPClient, password: String) -> anyhow::Result<()> {
    // 1) ask web server for JWT
    let res = client.get_jwt(password).await?;
    let token = AuthToken::new(&res);
    Ok(token.write_user_token()?)
}

/// Releases JWT
fn logout() -> anyhow::Result<()> {
    Ok(AuthToken::remove_user_token()?)
}

/// Shows stored JWT on stdout
fn show() -> anyhow::Result<()> {
    // we do not care if jwt() fails or not. If there is something to print, show it otherwise
    // stay silent
    if let Some(token) = AuthToken::find() {
        println!("{}", token.as_str());
    }

    Ok(())
}
