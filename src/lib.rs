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
#[derive(Default, Debug, Copy, Clone)]
pub struct Extended {
    #[default(true)]
    pub full: bool,

    #[default(false)]
    pub title: bool,
    #[default(false)]
    pub slug: bool,
    #[default(false)]
    pub overview: bool,
    #[default(false)]
    pub metadata: bool,
    #[default(false)]
    pub theater: bool,
    #[default(false)]
    pub genres: bool,
    #[default(false)]
    pub tmdb: bool,
}


pub fn get_extended_parameter(extended: Extended) -> Result<String, &'static str> {
    if extended.full && (
        extended.title || extended.slug || extended.overview || extended.metadata || extended.theater || extended.genres || extended.tmdb
    ) {
        return Err("extended cannot be full and have another parameter");
    }
    let selected: Vec<&str> = Vec::with_capacity(8);
    if selected.full {selected.push("full");}
    if selected.title {selected.push("title");}
    if selected.slug {selected.push("slug");}
    if selected.overview {selected.push("overview");}
    if selected.metadata {selected.push("metadata");}
    if selected.theater {selected.push("theater");}
    if selected.genres {selected.push("genres");}
    if selected.tmdb {selected.push("tmdb");}
    
    // FIXME: let result = selected.into_iter().collect::<Vec<&str>>.join(",");
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

#[derive(Default, Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct SeasonEpisode {
    pub number: u16,
}


#[derive(Default, Debug, Clone)]
pub struct Season {
    pub number: u16,
    pub episodes: Vec<SeasonEpisode>,
}

#[derive(Default, Debug, Clone)]
pub struct StandardMediaObject {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
    pub seasons: Option<Vec>,
    pub episodes: Option<SeasonVec>,
}

#[derive(Default, Debug, Clone)]
pub struct Episode {
    pub ids: Ids,
    // pub watched_at: DateTime,
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

// TODO: images, PIN for devices with limited UI, calendar (next 33 days, monthly)

#[derive(Default, Debug, Clone)]
pub struct Rank {
    r#type: String,
    votes: u32,
}

#[derive(Default, Debug, Clone)]
pub struct Rating {
    pub rating: f32,
    pub votes: u32,
}

#[derive(Default, Debug, Clone)]
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

// TODO: find by file
// TODO: find random
