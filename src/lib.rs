use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

pub mod anime;
pub mod auth;
pub mod calendar;
pub mod error;
pub mod images;
pub mod movie;
pub mod pagination;
pub mod pin;
pub mod rate_limit;
pub mod request;
pub mod response;
pub mod search;
pub mod show;
pub mod sync;
pub mod user;

#[derive(Debug)]
pub enum SimklError {
    /// JSON serialization/deserialization errors
    Json(serde_json::Error),
    /// Invalid URL
    InvalidUrl(url::ParseError),
    /// Missing or wrong parameters
    InvalidParameters(String),
    /// Parse error with the response
    ParseError(String),
}

impl fmt::Display for SimklError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimklError::Json(err) => write!(f, "Erreur JSON: {}", err),
            SimklError::InvalidUrl(err) => write!(f, "URL invalide: {}", err),
            SimklError::InvalidParameters(msg) => write!(f, "Paramètres invalides: {}", msg),
            SimklError::ParseError(msg) => write!(f, "Erreur de parsing: {}", msg),
        }
    }
}

impl std::error::Error for SimklError {}

impl From<serde_json::Error> for SimklError {
    fn from(err: serde_json::Error) -> Self {
        SimklError::Json(err)
    }
}

impl From<url::ParseError> for SimklError {
    fn from(err: url::ParseError) -> Self {
        SimklError::InvalidUrl(err)
    }
}

/// API URL, queries will need the following headers:
/// * `Content-Type: application/json`
/// * `simkl-api-key: <client_id>` (listed under your Simkl applications)
///
/// Full API doc is available at [https://simkl.docs.apiary.io/](https://simkl.docs.apiary.io/)
pub const API_URL: &str = "https://api.simkl.com";
pub const OAUTH_URL: &str = "https://simkl.com/oauth/authorize";
pub const TOKEN_URL: &str = "https://api.simkl.com/oauth/token";

/// By default methods are not returnig additional data for movies, anime, show etc. They return minimal info you need
/// to match in the local database. But, if you need more information just add `extended={fields}`` to the URL.
/// Will be in lower case
#[derive(Debug, Copy, Clone)]
pub struct Extended {
    pub full: bool,

    pub title: bool,
    pub slug: bool,
    pub overview: bool,
    pub metadata: bool,
    pub theater: bool,
    pub genres: bool,
    pub tmdb: bool,
}

impl Default for Extended {
    fn default() -> Self {
        Self {
            full: true,
            title: false,
            slug: false,
            overview: false,
            metadata: false,
            theater: false,
            genres: false,
            tmdb: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum MediaType {
    Movie,
    Show,
    Anime,
    Episode,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MediaType::Movie => "movie",
            MediaType::Show => "show",
            MediaType::Anime => "anime",
            MediaType::Episode => "episode",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "movie" => Ok(MediaType::Movie),
            "show" => Ok(MediaType::Show),
            "anime" => Ok(MediaType::Anime),
            "episode" => Ok(MediaType::Episode),
            _ => Err(()),
        }
    }
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Movie => "movie",
            MediaType::Show => "show",
            MediaType::Anime => "anime",
            MediaType::Episode => "episode",
        }
    }
}

pub fn get_extended_parameter(extended: Extended) -> Result<String, &'static str> {
    if extended.full
        && (extended.title
            || extended.slug
            || extended.overview
            || extended.metadata
            || extended.theater
            || extended.genres
            || extended.tmdb)
    {
        return Err("extended cannot be full and have another parameter");
    }
    let mut selected: Vec<&str> = Vec::with_capacity(8);
    if extended.full {
        selected.push("full");
    }
    if extended.title {
        selected.push("title");
    }
    if extended.slug {
        selected.push("slug");
    }
    if extended.overview {
        selected.push("overview");
    }
    if extended.metadata {
        selected.push("metadata");
    }
    if extended.theater {
        selected.push("theater");
    }
    if extended.genres {
        selected.push("genres");
    }
    if extended.tmdb {
        selected.push("tmdb");
    }

    let result = selected.into_iter().collect::<Vec<&str>>().join(",");
    Ok(result)
}

/// Some the endpoints are paginated. Endpoints which have 📄 Pagination will load 1 page of 10 items by default. If
/// you want to change this, append query string `?page={page}&limit={limit}` to the URL.
///
/// additionally, all paginated endpoints will return these HTTP headers as well
///
/// | Header | Value |
/// |--------|-------|
/// | X-Pagination-Page | Current page |
/// | X-Pagination-Limit | Items per page |
/// | X-Pagination-Page-Count | Total number of pages |
/// | X-Pagination-Item-Count | Total number of items |
pub fn get_pagination_parameter(page: u16, limit: u16) -> String {
    let mut result = String::with_capacity(32);
    result.push_str("?page=");
    result.push_str(&page.to_string());
    result.push_str("&limit=");
    result.push_str(&limit.to_string());
    result
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct MediaIds {
    pub simkl: Option<u32>,
    pub slug: Option<String>,
    pub imdb: Option<String>,
    // TV only
    pub tmdb: Option<u32>,
    // anime only
    pub mal: Option<u32>,
    pub anilist: Option<u32>,
    pub anidb: Option<u32>,
    pub tvdb: Option<u32>,
    pub kitsu: Option<u32>,
}

impl MediaIds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_simkl(mut self, id: u32) -> Self {
        self.simkl = Some(id);
        self
    }

    pub fn with_imdb(mut self, id: String) -> Self {
        self.imdb = Some(id);
        self
    }

    pub fn with_tmdb(mut self, id: u32) -> Self {
        self.tmdb = Some(id);
        self
    }

    pub fn with_mal(mut self, id: u32) -> Self {
        self.mal = Some(id);
        self
    }

    pub fn has_any_id(&self) -> bool {
        self.simkl.is_some()
            || self.imdb.is_some()
            || self.tmdb.is_some()
            || self.mal.is_some()
            || self.anilist.is_some()
            || self.anidb.is_some()
            || self.tvdb.is_some()
            || self.kitsu.is_some()
    }

    pub fn primary_id(&self) -> Option<String> {
        if let Some(id) = self.simkl {
            Some(id.to_string())
        } else if let Some(ref id) = self.imdb {
            Some(id.clone())
        } else {
            self.tmdb.map(|id| id.to_string())
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SeasonEpisode {
    pub number: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Season {
    pub number: u16,
    pub episodes: Vec<SeasonEpisode>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StandardMediaObject {
    pub title: String,
    pub year: u16,
    pub ids: MediaIds,
    pub seasons: Option<Vec<Season>>,
    pub episodes: Option<Vec<SeasonEpisode>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Episode {
    pub title: String,
    pub year: Option<u32>,
    pub released: Option<String>,
    pub pk: Option<u32>,
    pub ids: Option<MediaIds>,
    pub season: u32,
    pub episode: u32,
    pub watched: Option<bool>,
    pub last_watched_at: Option<String>,
    pub watchers: Option<u32>,
    pub plays: Option<u32>,
    pub img: Option<String>,
    pub overview: Option<String>,
    pub rating: Option<f32>,
    pub votes: Option<u32>,
    pub runtime: Option<u32>,
    pub user_rating: Option<u8>,
}

pub fn get_auth_url(client_id: &str, redirect_url: &str) -> String {
    let mut result = String::with_capacity(128);
    result.push_str(API_URL);
    result.push_str("oauth/authorize?response_type=code&redirect_uri=");
    result.push_str(redirect_url);
    result.push_str("&client_id=");
    result.push_str(client_id);
    result
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Rank {
    r#type: String,
    votes: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Rating {
    pub rating: Option<f32>,
    pub votes: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Ratings {
    id: u16,
    link: String,
    rank: Rank,
    simkl: Rating,
    imdb: Option<Rating>,
    has_trailer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirInfo {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
}

/// Get rating (token not required)
pub fn get_rating() -> Rating {
    todo!()
}

// TODO: find random
