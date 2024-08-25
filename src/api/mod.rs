
use std::{borrow::Cow, io::{self, BufRead, BufReader, Write}, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}, sync::{Arc, Mutex, RwLock}, thread};

use data::SpotifyAPIData;
use librespot::{
    core::{
        config::SessionConfig,
        mercury::MercuryError,
        session::{Session, SessionError}, Error,
    },
    discovery::{Credentials, DeviceType, Discovery},
};
use providers::{oauth2::{SpotifyAPIOAuthClient, SpotifyAPIOAuthError, SpotifyAPIOAuthProvider}, SpotifyAPIAuthProvider};
use rand::Rng;
use sha1::{digest::consts::U16, Digest, Sha1};
use spotify_rs::{auth::Token, client::{self, Client}};
use strum_macros::Display;
use futures::{StreamExt, TryFutureExt};
use tracing::{error, info};
use url::Url;

pub mod providers;
pub mod data;

use crate::{constants::{SPOTIFY_CLIENT_ID, SPOTIFY_DEVICE_NAME, SPOTIFY_OAUTH_AUTHORISE_URL, SPOTIFY_OAUTH_TOKEN_URL, SPOTIFY_SCOPES}, SpotifyAPILock};

#[derive(Debug)]
pub struct SpotifyAPI {
    pub busy_flags: SpotifyAPIBusyFlags,

    pub provider: SpotifyAPIOAuthProvider,

    pub data: Option<SpotifyAPIData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpotifyAPICredentials {
    pub username: String,
    pub password: String,
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    pub struct SpotifyAPIBusyFlags: u8 {
        const Busy = 0x01;
        const BusyLoggingIn = 0x02;
        const BusyFetching = 0x03;
    }
}

#[derive(Debug, Display, Default)]
pub enum SpotifyAPIError {
    #[default]
    Default,

    Unknown(&'static str),

    ValidationFailure(&'static str),
    CredentialsFailure(&'static str),
    BadOperation(&'static str),

    TokenFailure(MercuryError),
    SessionConnectFailure(Error),

    OAuth2Error(SpotifyAPIOAuthError)
}

impl Default for SpotifyAPI {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SpotifyAPIWrapper {
    pub async_mutex: tokio::sync::Mutex<SpotifyAPI>,
    pub sync_mutex: std::sync::Mutex<SpotifyAPI>,
}

impl SpotifyAPIWrapper {
    async fn lock_async(&self) -> tokio::sync::MutexGuard<'_, SpotifyAPI> {
        self.async_mutex.lock().await
    }

    fn lock_sync(&self) -> std::sync::MutexGuard<'_, SpotifyAPI> {
        self.sync_mutex.lock().unwrap()
    }
}

impl SpotifyAPI {
    pub fn new() -> Self {
        let provider = SpotifyAPIOAuthProvider::new();

        SpotifyAPI {
            provider,
            busy_flags: SpotifyAPIBusyFlags::empty(),
            data: None
        }
    }

    pub fn is_authorised(&self) -> bool {
        matches!(self.provider.client, Ok(SpotifyAPIOAuthClient::Authenticated(_)))
    }

    pub async fn create_client(&mut self) -> Result<SpotifyAPIOAuthClient, SpotifyAPIOAuthError> {
        let (code, csrf_state) = self.provider.wait_for_oauth_code().await?;
        self.provider.authenticate_client_from_code(code, csrf_state).await
    }

    pub fn get_auth_error(&self) -> Option<SpotifyAPIError> {
        self.provider.get_auth_error().map(SpotifyAPIError::OAuth2Error)
    }

    pub fn login(api: SpotifyAPILock) {
        let mut api_unlocked = api.lock().unwrap();

        let auth_url = api_unlocked.provider.auth_url.as_str();

        info!("Open this link in your web browser to continue:\n\n{}\n", auth_url);
        open::that(auth_url);

        api_unlocked.busy_flags.set(SpotifyAPIBusyFlags::BusyLoggingIn, true);

        let api_task_arc = Arc::clone(&api);

        thread::spawn(move || {
            let mut api_unlocked = api_task_arc.lock().unwrap();

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let client = api_unlocked.create_client().await;
                api_unlocked.provider.client = client;
            });
        });

        api_unlocked.busy_flags.remove(SpotifyAPIBusyFlags::BusyLoggingIn);
    }

    pub fn logout(&mut self) {
        info!("Logged out.");

        if let Some(new_provider) = self.provider.logout() {
            self.provider = new_provider;
        }
    }
}
