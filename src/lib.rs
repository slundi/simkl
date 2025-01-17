use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod calendar;
pub mod images;
pub mod pin;
pub mod search;
pub mod sync;
pub mod user;

/// API URL, queries will need the following headers:
/// * `Content-Type: application/json`
/// * `simkl-api-key: <client_id>` (listed under your Simkl applications)
///
/// Full API doc is available at [https://simkl.docs.apiary.io/](https://simkl.docs.apiary.io/)
pub const API_URL: &str = "https://api.simkl.com/";

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[repr(u8)]
pub enum Type {
    Tv,
    Anime,
    Movie,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MovieGenre {
    action,
    adventure,
    animation,
    comedy,
    crime,
    documentary,
    drama,
    erotica,
    Family,
    fantasy,
    foreign,
    history,
    horror,
    music,
    mystery,
    romance,
    science_fiction,
    thriller,
    tv_movie,
    war,
    western,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TvGenre {
    action,
    adventure,
    animation,
    awards_show,
    children,
    comedy,
    crime,
    documentary,
    drama,
    erotica,
    family,
    fantasy,
    food,
    game_show,
    history,
    home_and_garden,
    horror,
    indie,
    korean_drama,
    martial_arts,
    mini_series,
    musical,
    mystery,
    news,
    podcast,
    reality,
    romance,
    science_fiction,
    soap,
    special_interest,
    sport,
    suspense,
    talk_show,
    thriller,
    travel,
    war,
    western,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AnimeGenre {
    action,
    adventure,
    cars,
    comedy,
    dementia,
    demons,
    drama,
    ecchi,
    fantasy,
    game,
    harem,
    historical,
    horror,
    josei,
    kids,
    magic,
    martial_arts,
    mecha,
    military,
    music,
    mystery,
    parody,
    police,
    psychological,
    romance,
    samurai,
    school,
    sci_fi,
    seinen,
    shoujo,
    shoujo_ai,
    shounen,
    shounen_ai,
    slice_of_life,
    space,
    sports,
    super_power,
    supernatural,
    thriller,
    vampire,
    yaoi,
    yuri,
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
#[repr(u8)]
pub enum AnimeType {
    #[serde(rename(deserialize = "tv"))]
    Tv,
    #[serde(rename(deserialize = "special"))]
    Special,
    #[serde(rename(deserialize = "ova"))]
    Ova,
    #[serde(rename(deserialize = "movie"))]
    Movie,
    // will be "music video"
    #[serde(rename(deserialize = "music video"))]
    MusicVideo,
    #[serde(rename(deserialize = "ona"))]
    Ona,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Ids {
    pub simkl: Option<u32>,
    pub imdb: Option<String>,
    pub tmdb: Option<u32>,
    // TV only
    pub tvdb: Option<u32>,
    // anime only
    pub mal: Option<u32>,
    pub anidb: Option<u32>,
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
    pub ids: Ids,
    pub seasons: Option<Vec<Season>>,
    pub episodes: Option<Vec<SeasonEpisode>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Episode {
    pub ids: Ids,
    pub watched_at: DateTime<Utc>,
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

/// Get rating (token not required)
pub fn get_rating() -> Rating {
    todo!()
}

// TODO: find random
