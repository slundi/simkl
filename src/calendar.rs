use crate::{AnimeType, Rating, Ratings};
use chrono::{Date, DateTime};

pub const AIRING_NEXT_TV_URL: &str = "https://data.simkl.in/calendar/tv.json";
pub const AIRING_NEXT_ANIME_URL: &str = "https://data.simkl.in/calendar/anime.json";
pub const AIRING_NEXT_MOVIE_URL: &str = "https://data.simkl.in/calendar/movie_release.json";

#[repr(u8)]
pub enum Type {
    Tv,
    Anime,
    Movie,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RatingsItem {
    pub simkl: Rating,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct IdsItem {
    pub simkl_id: u32,
    pub slug: String,
    pub tmdb: u32,
    pub imdb: Option<String>,
    pub mal: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EpisodeItem {
    pub season: Option<u16>,
    pub episode: u16,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CalendarItem {
    pub title: String,
    pub poster: String,
    pub date: DateTime,
    pub release_date: Date,
    pub rank: u32,
    pub ratings: RatingItem,
    pub url: String,
    pub ids: IdsItem,
    pub episode: Option<EpisodeItem>,
    pub anime_type: Option<AnimeType>,
}

/// Get monthly items.
///
/// Parameters:
/// * year: 4 digits format (example: `2025`)
/// * month: from 1 to 12 where 1 is january
pub fn get_monthly_request(what: Type, year: u16, month: u8) -> String {
    let mut result = String::from(API_URL);
    result.push_str("calendar/");
    result.push_str(&year.to_string());
    result.push('/');
    result.push_str(&month.to_string());
    match what {
        Type::Tv => result.push_str("/tv.json"),
        Type::Anime => result.push_str("/anime.json"),
        Type::Movie => result.push_str("/movie_release.json"),
    }
    result
}
