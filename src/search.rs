use crate::{AnimeGenre, Extended, MovieGenre, TvGenre, API_URL};

pub struct FindByFilePayload {
    // Try to find the file from the filename, example:
    /// `Were.The.Fugawis.S01E01E02.WS.DSR.x264-NY2.mkv`
    pub file: String,
    /// Some filenames consist of 2 or more parts. If you want to get info about second part for example you can pass 2 to this parameter
    pub part: Option<u8>,
}

/// Find by file URL, use `FindByFilePayload` with this
pub const FIND_BY_FILE_URL: &str = "https://api.simkl.com/search/file";

/// Struct used to build the payload of a search by ID
#[derive(Default, Debug, Clone, PartialEq)]
pub struct IdLookup {
    pub simkl: Option<u32>,
    pub hulu: Option<u32>,
    pub netflix: Option<u32>,
    pub mal: Option<u32>,
    pub tvdb: Option<u32>,
    pub tmdb: Option<u32>,
    pub imdb: Option<String>,
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
}

pub fn get_search_by_id_request(payload: IdLookup, client_id: String) -> String {
    let mut result = String::from(API_URL);
    result.push_str("search/id?client_id=");
    result.push_str(&client_id);
    if let Some(id) = payload.simkl {
        result.push_str("&simkl=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.hulu {
        result.push_str("&hulu=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.netflix {
        result.push_str("&netflix=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.mal {
        result.push_str("&mal=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.tvdb {
        result.push_str("&tvdb=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.tmdb {
        result.push_str("&tmdb=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.imdb {
        result.push_str("&imdb=");
        result.push_str(&id);
    }
    if let Some(id) = payload.anidb {
        result.push_str("&anidb=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.crunchyroll {
        result.push_str("&crunchyroll=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.anilist {
        result.push_str("&anilist=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.kitsu {
        result.push_str("&kitsu=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.livechart {
        result.push_str("&livechart=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.anisearch {
        result.push_str("&anisearch=");
        result.push_str(&id.to_string());
    }
    if let Some(id) = payload.animeplanet {
        result.push_str("&animeplanet=");
        result.push_str(&id.to_string());
    }
    if let Some(t) = payload.r#type {
        result.push_str("&type=");
        result.push_str(&t);
    }
    if let Some(t) = payload.title {
        result.push_str("&title=");
        result.push_str(&t);
    }
    if let Some(y) = payload.year {
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
pub fn get_search_request(
    text: String,
    r#type: String,
    extended: Option<Extended>,
    client_id: String,
) -> String {
    todo!()
}

pub struct FindRandomPayload {
    pub service: String,
    pub r#type: crate::Type, // v , anime , movie
    pub movie_genre: MovieGenre,
    pub tv_genre: TvGenre,
    pub anime_genre: AnimeGenre,
    /// Max value is 10. Random search for TV Shows and Movies will be performed using IMDB ratings. Anime are based on
    /// MAL ratings.
    ///
    /// Example: `5`. Default: `1`.
    pub rating_from: u8,
    /// Example: `10`
    pub rating_to: Option<u8>,
    /// Maximum rank allowed. Example: `2000`.
    pub rank_limit: u32,
    pub year_from: Option<u16>,
    pub year_to: Option<u16>,
    pub limit: u16,
}

/// if you want to find random item based on your filters. If Token is passed, wacthed items will be excluded.
///
/// Examples:
/// * `https://api.simkl.com/search/random?service=simkl&type=tv&genre=comedy&rating_from=5&rating_to=10&year_from=2004&year_to=2010&limit=10&client_id=***`
/// * `https://api.simkl.com/search/random/netflix/?rating_from=5&rating_to=10&year_from=2008&year_to=2015&genre=science-fiction&client_id=***`
/// * `https://api.simkl.com/search/random/?rating_from=5&rating_to=10&year_from=2008&year_to=2015&genre=science-fiction&client_id=***`
pub fn get_find_random_request(payload: FindRandomPayload, client_id: String) -> String {
    let mut result = String::from(API_URL);
    result.push_str("search/random/");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_request() {
        let payload = IdLookup {
            simkl: Some(123),
            ..Default::default()
        };
        let client_id = String::from("azerty123456");
        assert_eq!(
            get_search_by_id_request(payload.clone(), client_id),
            "https://api.simkl.com/search?client_id=azerty123456&simkl=123"
        );
    }
}
