use std::{borrow::{Borrow, Cow}, cell::RefCell, default, fmt::Display, io::{BufRead, Write}, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}, rc::Rc, sync::{Arc, Mutex}};

use std::io;
use std::io::BufReader;
use rand::Rng;
use spotify_rs::{auth::{self, NoVerifier, PkceVerifier, Token, UnAuthenticated}, client::Client, AuthCodeFlow, AuthCodePkceClient, AuthCodePkceFlow, RedirectUrl};
use strum_macros::Display;
use tracing::info;
use librespot::{core::SessionConfig, protocol::authentication::AuthenticationType};
use url::Url;

use crate::constants::{SPOTIFY_CLIENT_ID, SPOTIFY_OAUTH_AUTHORISE_URL, SPOTIFY_OAUTH_TOKEN_URL, SPOTIFY_SCOPES};

#[derive(Debug, Display, Default, Clone)]
pub enum SpotifyAPIOAuthError {
    #[default]
    Default,

    AuthenticationFailure(spotify_rs::Error),
    BadOperation(&'static str),

    OAuth2Failure(&'static str),
    OAuth2ServerBindFailure(Arc<io::Error>),
}

#[derive(Debug)]
pub enum SpotifyAPIOAuthClient {
    Unauthenticated(Client<UnAuthenticated, AuthCodePkceFlow, PkceVerifier>),
    Authenticated(Client<Token, AuthCodePkceFlow, NoVerifier>),
    Authenticating,
}

#[derive(Debug)]
pub struct SpotifyAPIOAuthProvider {
    pub client: Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError>,
    pub auth_url: Url,
    pub port: u16
}

impl Default for SpotifyAPIOAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl SpotifyAPIOAuthProvider {
    fn create_auth_flow() -> AuthCodePkceFlow {
        AuthCodePkceFlow::new(
            SPOTIFY_CLIENT_ID,
            SPOTIFY_SCOPES.split(",").collect::<Vec<&str>>()
        )
    }

    pub fn new() -> Self {
        let port = rand::thread_rng().gen_range(1000..65535);

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let redirect_uri = RedirectUrl::new(
            format!("http://{addr}/login")
        ).unwrap();

        let auth_code_flow = SpotifyAPIOAuthProvider::create_auth_flow();

        // Redirect the user to this URL to get the auth code and CSRF token
        let (client, auth_url) = AuthCodePkceClient::new(
            auth_code_flow,
            redirect_uri,
            true
        );

        Self {
            client: Ok(SpotifyAPIOAuthClient::Unauthenticated(client)),
            auth_url,
            port
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
            client.authenticate(&code, &csrf_state)
                .await
                .map(SpotifyAPIOAuthClient::Authenticated)
                .map_err(SpotifyAPIOAuthError::AuthenticationFailure)
        } else {
            unreachable!()
        }
    }

    pub async fn from_refresh_token(refresh_token: String) -> Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError> {
        let auth_code_flow: AuthCodePkceFlow = SpotifyAPIOAuthProvider::create_auth_flow();

        Client::from_refresh_token(
            AuthCodePkceFlow {
                client_id: SPOTIFY_CLIENT_ID.to_string(),
                scopes: auth_code_flow.scopes
            },
            true,
            refresh_token
        )
            .await
            .map(SpotifyAPIOAuthClient::Authenticated)
            .map_err(SpotifyAPIOAuthError::AuthenticationFailure)
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

                if !http_method.is_empty() && !http_uri.is_empty() {
                    if let Ok(http_uri_parsed) = Url::parse(&format!("http://localhost{}", http_uri)) {
                        match http_uri_parsed.path() {
                            "/login" => {
                                let code = http_uri_parsed.query_pairs()
                                    .find(|(key, value)| key == "code")
                                    .map(|(key, value)| value);

                                let csrf_token = http_uri_parsed.query_pairs()
                                    .find(|(key, value)| key == "csrf_token")
                                    .map(|(key, value)| value);

                                match (code, csrf_token) {
                                    (Some(code), Some(csrf_token)) => {
                                        info!("Got token from OAuth2 response.");

                                        stream.write_all(b"Successfully authenticated with Spotify. You can now close this page.").ok();
    
                                        return Ok((code.to_string(), csrf_token.to_string()));
                                    },
                                    _ => {
                                        let error = http_uri_parsed.query_pairs()
                                            .find(|q| q.0 == *"error").map(|e| e.1)
                                            .unwrap_or(Cow::from("Something went wrong, please retry the login flow again."));

                                        stream.write_all(error.as_bytes()).ok();
                                    }
                                }
                            },
                            _ => {
                                stream.write_all(b"404 Not Found").ok();
                            }
                        }
                    }
                }
            }
        }

        Err(SpotifyAPIOAuthError::OAuth2Failure("No response handled from OAuth2 server."))
    }

    pub fn logout(&mut self) -> Option<Self> {
        if matches!(self.client, Ok(SpotifyAPIOAuthClient::Authenticated(_))) {
            Some(SpotifyAPIOAuthProvider::new())
        } else {
            None
        }
    }
}