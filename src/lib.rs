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
};


pub fn get_extended parameter(extended: Extended) -> Result<String, &'static str> {
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
    
    let result = selected.into_iter().collect::<Vec<&str>>.join(",");
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
};

#[derive(Default, Debug, Clone)]
pub struct SeasonEpisode {
    pub number: u16,
};


#[derive(Default, Debug, Clone)]
pub struct Season {
    pub number: u16,
    pub episodes: Vec<SeasonEpisode>,
};

#[derive(Default, Debug, Clone)]
pub struct StandardMediaObject {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
    pub seasons: Option<Vec>,
    pub episodes: Option<SeasonVec>,
};

#[derive(Default, Debug, Clone)]
pub struct Episode {
    pub ids: Ids,
    // pub watched_at: DateTime,
};

pub fn get_auth_url(client_id: &str, redirect_url: &str) -> String {
    let mut result = String::with_capacity(128);
    result.push_str(API_URL);
    result.push_str("oauth/authorize?response_type=code&redirect_uri=");
    result.push_str(redirect_url);
    result.push_str("&client_id=")
    result.push_str(client_id);
    result
}

// TODO: images, PIN for devices with limited UI, calendar (next 33 days, monthly)

pub struct Rank {
    r#type: String,
    votes: u32,
};

pub struct Rating {
    pub rating: f32,
    pub votes: u32,
};

pub struct Ratings {
    id: u16,
    link: String,
    rank: Rank,
    simkl: Rating,
    imdb: Option<Rating>,
    has_trailer: bool,
};

/// Get rating (token not required)
pub fn get_rating() -> Rating {
    todo!()
}

/// Struct used to build the payload of a search by ID
pub struct IdLookup{
    pub simkl: Option<u32>,
    pub hulu: Option<u32>,
    pub netflix: Option<u32>,
    pub mal: Option<u32>,
    pub tvdb: Option<u32>,
    pub tmdb: Option<u32>,
    pub imdb: String,
    pub anidb: Option<u32>,
    pub crunchyroll: Option<u32>,
    pub anilist: Option<u32>,
    pub kitsu: Option<u32>,
    pub livechart: Option<u32>,
    pub anisearch: Option<u32>,
    pub animeplanet: Option<u32>,
    /// Possible values are: show, movie
    pub r#type: Option<String>,
    /// TV show, anime, or movie title. If this title has more then 1 item then null will be returned, add more fields
    /// to narrow down the search to 1 item(such as type,year etc.). Example: `The Walking Dead`.
    pub title: Option<String>,
    pub year: Option<u16>,
};

pub fn get_search_by_id_request(payload: IdLookup, client_id: String) -> String {
    let mut result = String::from(API_URL);
    result.push_str("search/id?client_id=");
    result.push_str(&client_id);
    if let Ok(id) = payload.simkl {
        result.push_str("&simkl=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.hulu {
        result.push_str("&hulu=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.netflix {
        result.push_str("&netflix=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.mal {
        result.push_str("&mal=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.tvdb {
        result.push_str("&tvdb=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.tmdb {
        result.push_str("&tmdb=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.imdb {
        result.push_str("&imdb=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.anidb {
        result.push_str("&anidb=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.crunchyroll {
        result.push_str("&crunchyroll=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.anilist {
        result.push_str("&anilist=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.kitsu {
        result.push_str("&kitsu=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.livechart {
        result.push_str("&livechart=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.anisearch {
        result.push_str("&anisearch=");
        result.push_str(&id.to_string());
    }
    if let Ok(id) = payload.animeplanet {
        result.push_str("&animeplanet=");
        result.push_str(&id.to_string());
    }
    if let Ok(t) = payload.type {
        result.push_str("&type=");
        result.push_str(&t);
    }
    if let Ok(t) = payload.title {
        result.push_str("&title=");
        result.push_str(&t);
    }
    if let Ok(y) = payload.year {
        result.push_str("&year=");
        result.push_str(&y.to_string());
    }
    result
}

/// Search items by title, sorted by relevance (what people search most). This method will respond with
/// `StandardMediaObject` + additional fields if `extended` parameter passed.
///
/// For `movies` or `anime` with movie type, tmdb id points to the movies section on TMDB site, otherwise to the TV
/// section.
///
/// Items with `endpoint_type = anime` has additional anime `type` key, see possible values for the `anime_type` key in
/// `StandardMediaObject`.
///
/// Page limit is 20, max items per page is 50.
pub get_search_request(text: String, type: String, extended: Option(Extended), client_id: String) -> String {
    todo!()
}

// TODO: find by file
// TODO: find random
