pub mod providers;
pub mod data;
pub mod error;
pub mod models;
pub mod enums;
pub mod utils;
pub mod constants;

use std::{borrow::Cow, collections::HashMap, rc::Rc, sync::{Arc, Mutex}, time::{Duration, SystemTime}};

use constants::{SPOTIFY_ACCOUNTS_URL, SPOTIFY_API_URL, SPOTIFY_CATEGORIES_INTERNAL, SPOTIFY_CATEGORY_ID_MADE_FOR_YOU};
use data::{SpotifyAPIData};
use easy_imgui::IntoCStr;
use easy_imgui_window::winit::event_loop::EventLoopProxy;
use enums::search::SpotifyAPISearchType;
use error::SpotifyAPIError;
use models::{recommendations::{BrowseRecommendationSections, BrowseRecommendations}, search::SearchResults, user::UserImpl as _};
use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, RefreshToken, StandardTokenResponse, TokenResponse};
use providers::oauth2::{SpotifyAPIOAuthClient, SpotifyAPIOAuthProvider};
use reqwest::Method;
use rspotify_model::{Category, CategoryPlaylists, FeaturedPlaylists, FullPlaylist, Page, PageCategory, PrivateUser, Recommendations, SearchResult, SimplifiedPlaylist};
use serde::Deserialize;
use tracing::{error, info, warn};
use url::Url;
use utils::prompt_open_url;

use crate::{constants::{UI_APP_NAME, UI_APP_VERSION, UI_DEFAULT_LOCALE}, event::{AppEvent, AppFetchType}, widget::Widget, WidgetRc};

#[derive(Debug)]
pub enum SpotifyAPIState {
    NotAuthenticated,
    Authenticating,
    Authenticated,
    LoggingIn,
    LoggedIn(SpotifyAPIData)
}

impl SpotifyAPIState {
    pub fn is_logged_in(&self) -> bool {
        matches!(self, Self::LoggedIn(_))
    }
 }

#[derive(Clone)]
pub struct SpotifyAPI {
    pub event_loop: Arc<EventLoopProxy<AppEvent>>,

    pub state: Arc<Mutex<Result<SpotifyAPIState, SpotifyAPIError>>>,

    pub provider: Arc<tokio::sync::Mutex<SpotifyAPIOAuthProvider>>,

    pub client: reqwest::Client,
}

impl SpotifyAPI {
    pub fn create_new_provider(refresh_token: Option<String>) -> SpotifyAPIOAuthProvider {
        SpotifyAPIOAuthProvider::new(refresh_token)
    }

    pub fn new(event_loop: Arc<EventLoopProxy<AppEvent>>, refresh_token: Option<String>) -> Self {
        let provider = Arc::new(tokio::sync::Mutex::new(SpotifyAPI::create_new_provider(refresh_token.clone())));
        let state = Arc::new(Mutex::new(Ok(SpotifyAPIState::NotAuthenticated)));

        let user_agent = format!("{}/{}", UI_APP_NAME, UI_APP_VERSION.unwrap_or("0.0.0"));

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        if refresh_token.clone().is_some() {
            event_loop.send_event(AppEvent::Login).ok();
        }

        SpotifyAPI {
            event_loop,

            state,
            provider,
            client
        }
    }

    pub fn get_state_error(&self) -> Option<SpotifyAPIError> {
        let unlocked = self.state
            .lock()
            .unwrap();

        let opt_err = unlocked
            .as_ref()
            .err();

        opt_err.cloned()
    }

    pub fn is_logging_in(&self) -> bool {
        matches!(*self.state.lock().unwrap(), Ok(SpotifyAPIState::Authenticating) | Ok(SpotifyAPIState::LoggingIn))
    }

    pub fn is_logged_in(&self) -> bool {
        matches!(*self.state.lock().unwrap(), Ok(SpotifyAPIState::LoggedIn(_)))
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(*self.state.lock().unwrap(), Ok(SpotifyAPIState::Authenticated))
    }

    pub fn is_not_authenticated(&self) -> bool {
        !(
            self.is_authenticated() &&
            self.is_logged_in() &&
            self.is_logging_in()
        )
    }

    pub fn is_authenticating(&self) -> bool {
        matches!(*self.state.lock().unwrap(), Ok(SpotifyAPIState::Authenticating))
    }

    pub fn state(&self) -> Option<SpotifyAPIData> {
        let unlocked_state = self.state.lock().unwrap();
        let state_result = unlocked_state.as_ref();

        match state_result {
            Ok(SpotifyAPIState::LoggedIn(data)) => Some(data.clone()),
            _ => None
        }
    }

    pub async fn token(&self, should_refresh: Option<bool>) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, SpotifyAPIError> {
        let mut provider = self.provider.lock().await;

        provider.token(should_refresh)
            .await
            .map_err(SpotifyAPIError::OAuth2Error)
    }

    pub async fn access_token(&self) -> Result<String, SpotifyAPIError> {
        let is_state_err = self.state.lock().unwrap().is_err();

        Ok(self.token(Some(is_state_err))
            .await?
            .access_token()
            .secret()
            .to_string())
    }

    pub async fn refresh_token(&self) -> Result<Option<String>, SpotifyAPIError> {
        Ok(self.token(Some(false))
            .await?
            .refresh_token().map(|t| t.secret().to_string()))
    }

    pub async fn request<R: for<'a> Deserialize<'a>>(&self, method: Method, route: String, query: Option<HashMap<&str, Option<&String>>>) -> Result<R, SpotifyAPIError> {
        let time_start = SystemTime::now();

        let mut url = Url::parse(SPOTIFY_API_URL)
            .unwrap();
        url = url.join(&route[1..]).unwrap();

        let access_token = self.access_token().await?;
        let refresh_token = self.refresh_token().await?;

        let response = self.client.request(method.clone(), url.as_str())
            .bearer_auth(access_token)
            .query(&query.unwrap_or([].into()))
            .send()
            .await
            .map_err(|e| SpotifyAPIError::RequestError(Arc::new(e)))?;

        let status = response.status();
        let response_url = response.url().clone();

        let time_end = SystemTime::now();
        let time_diff = time_end.duration_since(time_start).unwrap().as_nanos() as f32 / 1_000_000.0;

        let response_error = response.error_for_status_ref()
            .map(|_| ())
            .map_err(|e| SpotifyAPIError::RequestError(Arc::new(e)))
            .err();

        let path = response_url.path().to_string();

        let query = if let Some(query) = response_url.query() {
            let clean_query = urlencoding::decode(query)
                .unwrap_or(Cow::from(query));

            format!("?{}", clean_query)
        } else {
            "".to_string()
        };

        let handle_error = |err, text: Option<String>| {
            error!(
                "{} {}{}: {} in {:.2}ms {:#?}: {:#}",
                method.clone(),
                path,
                query,
                status.as_str(),
                time_diff,
                err,
                text.unwrap_or("No message from server.".to_string())
            );

            Err(err)
        };

        let text = match response.text().await
            .map_err(|e| SpotifyAPIError::RequestError(Arc::new(e)))
        {
            Ok(t) => t,
            Err(err) => return handle_error(err, None)?
        };

        let data = match serde_json::de::from_str::<R>(&text)
            .map_err(|e| SpotifyAPIError::SerdeError(Arc::new(e)))
        {
            Ok(t) => t,
            Err(err) => return handle_error(err, Some(text))?
        };

        if response_error.is_some() {
            let pretty_text = serde_json::ser::to_string_pretty(&text).unwrap_or(text);

            return handle_error(
                response_error.unwrap(),
                Some(pretty_text)
            )?
        } else {
            info!(
                "{} {}{}: {} in {}ms",
                method.clone(),
                path,
                query,
                status.as_str(),
                time_diff
            );
        }

        self.event_loop.send_event(AppEvent::StoreToken(refresh_token)).ok();

        Ok(data)
    }

    pub async fn get_current_user_profile(&self) -> Result<PrivateUser, SpotifyAPIError> {
        self.request::<PrivateUser>(Method::GET, "/me".to_string(), None).await
    }

    pub async fn search(&self, query: String, search_types: Option<SpotifyAPISearchType>, limit: Option<u32>) -> Result<SearchResults, SpotifyAPIError> {
        let search_type = search_types
            .unwrap_or(SpotifyAPISearchType::all())
            .to_string();

        let limit = limit.map(|x| x.to_string());

        let query = utils::create_hashmap(&[
            ("query", Some(&query)),
            ("type", Some(&search_type)),
            ("limit", limit.as_ref())
        ]);

        self.request::<SearchResults>(Method::GET, "/search".to_string(), Some(query)).await
    }

    pub async fn get_browse_categories(&self, locale: String, limit: Option<u32>) -> Result<Page<Category>, SpotifyAPIError> {
        let limit = limit.map(|x| x.to_string());

        let query = utils::create_hashmap(&[
            ("locale", Some(&locale)),
            ("limit", limit.as_ref())
        ]);

        self.request::<PageCategory>(Method::GET, "/browse/categories".to_string(), Some(query)).await
            .map(|r| r.categories)
    }

    pub async fn get_featured_playlists(&self, locale: String, limit: Option<u32>) -> Result<Page<SimplifiedPlaylist>, SpotifyAPIError> {
        let limit = limit.map(|x| x.to_string());

        let query = utils::create_hashmap(&[
            ("locale", Some(&locale)),
            ("limit", limit.as_ref())
        ]);

        self.request::<FeaturedPlaylists>(Method::GET, "/browse/featured-playlists".to_string(), Some(query)).await
            .map(|r| r.playlists)
    }

    pub async fn get_browse_recommendations(&self, locale: String) -> Result<BrowseRecommendations, SpotifyAPIError> {
        let browse_categories = self.get_browse_categories(locale.clone(), Some(20)).await?;
        let featured_playlists = self.get_featured_playlists(locale.clone(), Some(20)).await?;

        let mut categories = browse_categories.items.clone();

        for category in SPOTIFY_CATEGORIES_INTERNAL {
            categories.insert(0, self.get_category(
                category.to_string(),
                locale.clone()
            ).await?);
        }

        let mut recommendations = BrowseRecommendations {
            categories,
            featured: featured_playlists,

            sections: Some(BrowseRecommendationSections::Fetching)
        };

        recommendations.generate(self, locale, None).await;

        Ok(recommendations)
    }

    pub async fn get_category(&self, category_id: String, locale: String) -> Result<Category, SpotifyAPIError> {
        let query = utils::create_hashmap(&[
            ("locale", Some(&locale))
        ]);

        self.request::<Category>(
            Method::GET,
            format!("/browse/categories/{}", category_id),
            Some(query)
        )
            .await
    }

    pub async fn get_category_playlists(&self, category_id: String, limit: Option<u32>) -> Result<Page<SimplifiedPlaylist>, SpotifyAPIError> {
        let limit = limit.map(|x| x.to_string());

        let query = utils::create_hashmap(&[
            ("limit", limit.as_ref())
        ]);

        self.request::<CategoryPlaylists>(
            Method::GET,
            format!("/browse/categories/{}/playlists", category_id),
            Some(query)
        )
            .await
            .map(|r| r.playlists)
    }

    async fn fetch_data(&self, locale: String) -> Result<(), SpotifyAPIError> {
        let is_already_authenticated = self.is_logged_in();

        // Skip changing the state to LoggingIn if we're already authenticated.
        //
        // When we re-fetch data after authenticating, we will
        // see occasional flickers as the state changes from LoggingIn
        // to LoggedIn.
        if !is_already_authenticated {
            *self.state.lock().unwrap() = Ok(SpotifyAPIState::LoggingIn);
        }

        // Fetch stuff here
        let profile = self.get_current_user_profile().await?;

        *self.state.lock().unwrap() = Ok(SpotifyAPIState::LoggedIn(SpotifyAPIData {
            profile: Some(profile)
        }));

        // Only log that we've logged in if we don't have a session yet:
        // Routine calls to fetch_data would log this too frequently.
        if !is_already_authenticated {
            if let Some(profile) = self.state().and_then(|s| s.profile) {
                info!(
                    "Logged into {} ({}).",
                    profile.name(),
                    profile.email_safe()
                );
            }

            self.event_loop.send_event(AppEvent::FirstTimeLogin).ok();
        }

        Ok(())
    }

    pub async fn fetch_data_wrapper(&self, locale: String) {
        match self.fetch_data(locale).await {
            Ok(_) => {},
            Err(err) => {
                error!("Failed to fetch data from Spotify: {:#?}", err);

                *self.state.lock().unwrap() = Err(err);
            }
        }
    }

    pub async fn login(&self, force: Option<bool>) {
        let mut provider = self.provider.lock().await;

        // If we don't have a refresh token ready
        // prompt the user to login through their browser.
        if force.unwrap_or(false) || provider.init_refresh_token.is_none() {
            let auth_url = provider.auth_url.as_str();

            prompt_open_url(auth_url.to_string());
        } else {
            info!("Authenticating with refresh token...");
        }

        *self.state.lock().unwrap() = Ok(SpotifyAPIState::Authenticating);

        match provider.update_client(force)
            .await
            .map_err(SpotifyAPIError::OAuth2Error)
        {
            Ok(_) => {
                info!("Authenticated with Spotify, logging in...");

                *self.state.lock().unwrap() = Ok(SpotifyAPIState::Authenticated);

                self.event_loop.send_event(AppEvent::Fetch(AppFetchType::All)).ok();
            },
            Err(err) => {
                error!("Failed to update API client: {:#?}", err);

                *self.state.lock().unwrap() = Err(err);
            }
        }
    }

    pub async fn logout(&self) {
        let cached_profile = self.state()
            .and_then(|s| s.profile);

        *self.state.lock().unwrap() = Ok(SpotifyAPIState::NotAuthenticated);

        *self.provider.lock().await = SpotifyAPIOAuthProvider::new(None);

        if let Some(profile) = cached_profile {
            info!(
                "Logged out of {} ({}).",
                profile.name(),
                profile.email.unwrap_or("no email".to_string())
            );
        }

        self.event_loop.send_event(AppEvent::StoreToken(None)).ok();
    }

    pub fn open_accounts_page(&self) -> std::io::Result<()> {
        prompt_open_url(SPOTIFY_ACCOUNTS_URL.to_string())
    }
}