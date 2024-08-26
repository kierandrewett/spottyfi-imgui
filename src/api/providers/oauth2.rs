use std::{borrow::{Borrow, Cow}, cell::RefCell, collections::HashSet, default, fmt::Display, io::{BufRead, Write}, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}, rc::Rc, sync::{Arc, Mutex}};

use std::io;
use std::io::BufReader;
use chrono::Duration;
use rand::Rng;
use oauth2::{basic::{BasicClient, BasicErrorResponse, BasicErrorResponseType, BasicRequestTokenError, BasicTokenResponse, BasicTokenType}, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, RefreshTokenRequest, RequestTokenError, Scope, StandardErrorResponse, StandardTokenResponse, TokenResponse, TokenUrl};
use strum_macros::Display;
use tracing::{error, info, warn};
use librespot::{core::SessionConfig, protocol::authentication::AuthenticationType};
use url::Url;

use crate::{api::constants::{SPOTIFY_CLIENT_ID, SPOTIFY_OAUTH_AUTHORISE_URL, SPOTIFY_OAUTH_TOKEN_URL, SPOTIFY_SCOPES}, constants::{UI_APP_NAME, UI_APP_VERSION}};

#[derive(Debug, Display, Default, Clone)]
pub enum SpotifyAPIOAuthError {
    #[default]
    Default,

    AuthenticationFailure(String),
    BadOperation(&'static str),

    ServerFailure(String),
    CodeExchangeFailure(Arc<RequestTokenError<
        oauth2::reqwest::AsyncHttpClientError,
        StandardErrorResponse<BasicErrorResponseType>,
    >>),
    OAuth2ServerBindFailure(Arc<io::Error>),
}

#[derive(Debug)]
pub enum SpotifyAPIOAuthClient {
    Unauthenticated(BasicClient),
    Authenticated(BasicClient),
    Authenticating,
}

#[derive(Debug)]
pub struct SpotifyAPIOAuthProvider {
    pub client: Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError>,
    pub token: Option<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>,
    pub pkce_verifier: PkceCodeVerifier,
    pub auth_url: Url,
    pub port: u16,
    pub init_refresh_token: Option<String>
}

impl Default for SpotifyAPIOAuthProvider {
    fn default() -> Self {
        Self::new(None)
    }
}

impl SpotifyAPIOAuthProvider {
    pub fn new(init_refresh_token: Option<String>) -> Self {
        let port = rand::thread_rng().gen_range(1000..65535);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let redirect_uri = format!("http://{addr}/login");

        let client =
            BasicClient::new(
                ClientId::new(SPOTIFY_CLIENT_ID.to_string()),
                None,
                AuthUrl::new(SPOTIFY_OAUTH_AUTHORISE_URL.to_string()).unwrap(),
                Some(TokenUrl::new(SPOTIFY_OAUTH_TOKEN_URL.to_string()).unwrap())
            )
                .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let scopes = SPOTIFY_SCOPES.split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| Scope::new(s.to_string()));

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .add_scopes(scopes)
            .url();

        Self {
            client: Ok(SpotifyAPIOAuthClient::Unauthenticated(client)),
            token: None,
            pkce_verifier,
            auth_url,
            port,
            init_refresh_token
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }

    pub fn get_auth_error(&self) -> Option<SpotifyAPIOAuthError> {
        let opt_err = self
            .client
            .as_ref()
            .err();

        opt_err.cloned()
    }

    pub async fn token(&mut self, should_refresh: Option<bool>) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, SpotifyAPIOAuthError> {
        match &self.client {
            Ok(SpotifyAPIOAuthClient::Authenticated(client)) => {
                if let Some(token) = self.token.clone() {
                    if should_refresh.unwrap_or(false) {
                        info!("Refreshing your access token...");

                        let refresh_token = token.refresh_token();

                        self.token = Some(client.exchange_refresh_token(refresh_token.unwrap())
                            .request_async(async_http_client)
                            .await
                            .map_err(|e| SpotifyAPIOAuthError::CodeExchangeFailure(Arc::new(e)))?);
                    }

                    return Ok(
                        self.token
                            .clone()
                            .unwrap()
                    )
                }

                Err(SpotifyAPIOAuthError::AuthenticationFailure("Not authenticated with Spotify.".to_string()))
            },
            Err(err) => Err(err.clone()),
            _ => Err(SpotifyAPIOAuthError::AuthenticationFailure("Not authenticated with Spotify.".to_string()))
        }
    }

    pub async fn authenticate_client_from_code(
        &mut self,
        code: String,
        csrf_state: String,
    ) -> Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError> {
        match &self.client {
            Ok(SpotifyAPIOAuthClient::Authenticated(_)) => {
                return Err(SpotifyAPIOAuthError::BadOperation("Cannot authenticate from an already authenticated client!"))
            }
            Ok(SpotifyAPIOAuthClient::Authenticating) => {
                return Err(SpotifyAPIOAuthError::BadOperation("Cannot authenticate with a pending operation!"))
            }
            Ok(SpotifyAPIOAuthClient::Unauthenticated(_)) => { }, // Handle unauthenticated clients below
            Err(err) => return Err(err.clone()), // Clone the error for returning
        }

        if let Ok(SpotifyAPIOAuthClient::Unauthenticated(client)) = std::mem::replace(
            &mut self.client,
            Ok(SpotifyAPIOAuthClient::Authenticating)
        ) {
            let code = AuthorizationCode::new(code);
            let pkce_verifier = PkceCodeVerifier::new(self.pkce_verifier.secret().to_string());

            self.init_refresh_token = None;

            let token_result = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request_async(async_http_client)
                .await
                .map_err(|e| SpotifyAPIOAuthError::CodeExchangeFailure(Arc::new(e)))?;

            self.token = Some(token_result);

            Ok(SpotifyAPIOAuthClient::Authenticated(client))
        } else {
            unreachable!()
        }
    }

    pub async fn authenticate_client_from_refresh_token(&mut self, refresh_token_secret: String) -> Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError> {
        let refresh_token = RefreshToken::new(refresh_token_secret);

        match &self.client {
            Ok(SpotifyAPIOAuthClient::Unauthenticated(client)) => {},
            Err(err) => return Err(err.clone()), // Clone the error for returning
            _ => {
                return Err(SpotifyAPIOAuthError::BadOperation("Cannot refresh client using an already authenticated client!"))
            }
        }

        if let Ok(SpotifyAPIOAuthClient::Unauthenticated(client)) = std::mem::replace(
            &mut self.client,
            Ok(SpotifyAPIOAuthClient::Authenticating)
        ) {
            let token_result = client.exchange_refresh_token(&refresh_token)
                .request_async(async_http_client)
                .await
                .map_err(|e| SpotifyAPIOAuthError::CodeExchangeFailure(Arc::new(e)))?;

            self.token = Some(token_result);

            Ok(SpotifyAPIOAuthClient::Authenticated(client))
        } else {
            unreachable!()
        }
    }

    pub async fn wait_for_oauth_code(&self) -> Result<(String, String), SpotifyAPIOAuthError> {
        let addr = &self.get_addr();

        let listener = TcpListener::bind(addr)
            .map_err(|e| SpotifyAPIOAuthError::OAuth2ServerBindFailure(Arc::new(e)))?;

        for mut stream in listener.incoming().flatten() {
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<String> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            if let Some(http_method_uri_line) = http_request.first().map(|ln| ln.split(" ").collect::<Vec<&str>>()) {
                let http_method = http_method_uri_line[0].trim().to_lowercase();
                let http_uri = http_method_uri_line[1].trim();

                let mut http_message = |status: &'static str, body: String| {
                    let safe_version = UI_APP_VERSION.unwrap_or("");

                    let message = format!("HTTP/1.1 {status}\n\
                        Content-Type: text/html\n\
                        Connection: keep-alive\n\
                        \n\
                        <html>
                        <head>
                            <title>{UI_APP_NAME} {safe_version}</title>
                            <style>html,body {{ font-family: system-ui, sans-serif; }}</style>
                        </head>
                        <body>
                            <center>
                                <h1>{status}</h1>
                                <p>{body}</p>

                                <hr>
                                <small>{UI_APP_NAME} {safe_version}</small>
                            </center>
                        </body>
                    ");

                    stream.write_all(message.as_bytes()).ok();
                };

                if !http_method.is_empty() && !http_uri.is_empty() {
                    if let Ok(http_uri_parsed) = Url::parse(&format!("http://localhost{}", http_uri)) {
                        match http_uri_parsed.path() {
                            "/login" => {
                                let code = http_uri_parsed.query_pairs()
                                    .find(|(key, value)| key == "code")
                                    .map(|(key, value)| value);

                                let csrf_state = http_uri_parsed.query_pairs()
                                    .find(|(key, value)| key == "state")
                                    .map(|(key, value)| value);

                                let mut throw_error = |message: String| {
                                    warn!(message);

                                    let http_error_message = format!("{}<br><br>Please try logging in again from the app.", message.clone());

                                    http_message("400 Bad Request", http_error_message);

                                    Err(SpotifyAPIOAuthError::ServerFailure(message.clone()))
                                };

                                match (code, csrf_state) {
                                    (Some(code), Some(csrf_state)) => {
                                        info!("Received token from OAuth2 auth request ({} bytes).", code.as_bytes().len());

                                        http_message(
                                            "200 OK",
                                            "Successfully authenticated with Spotify.<br><br>You can now close this page.".to_string()
                                        );

                                        return Ok((code.to_string(), csrf_state.to_string()));
                                    },
                                    (None, Some(_)) => return throw_error("No code parameter supplied with request.".to_string()),
                                    (Some(_), None) => return throw_error("No csrf_token parameter supplied with request.".to_string()),
                                    _ => {
                                        let error = http_uri_parsed.query_pairs()
                                            .find(|q| q.0 == *"error").map(|e| e.1)
                                            .unwrap_or(Cow::from("Something went wrong, please retry the login flow again."));

                                        return throw_error(error.clone().to_string());
                                    }
                                }
                            },
                            _ => {
                                http_message("404 Not Found", "404 Not Found".to_string());
                            }
                        }
                    }
                }
            }
        }

        Err(SpotifyAPIOAuthError::ServerFailure("No response handled from OAuth2 server.".to_string()))
    }

    pub async fn update_client(&mut self, force: Option<bool>) -> Result<(), SpotifyAPIOAuthError> {
        let client = if force.unwrap_or(false) || self.init_refresh_token.is_none() {
            let (code, csrf_state) = self.wait_for_oauth_code().await?;
            self.authenticate_client_from_code(code, csrf_state).await?
        } else {
            self.authenticate_client_from_refresh_token(self.init_refresh_token.clone().unwrap().to_string()).await?
        };

        self.client = Ok(client);

        Ok(())
    }
}