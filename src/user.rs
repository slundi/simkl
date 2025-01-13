use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{StandardMediaObject, API_URL};

/// User settings, requires those headers in your request:
/// * `Content-Type:application/json`
/// * `Authorization:Bearer [token]`
/// * `simkl-api-key:[client_id]`
///
/// Response will look like:
/// ``` json
/// {
///   "user": {
///     "name": "Andrew Masyk",
///     "joined_at": "2007-11-04T22:04:46Z",
///     "gender": "male",
///     "avatar": "https://graph.facebook.com/736253775/picture?width=100&height=100",
///     "bio": "I'm a founder and developer at simkl.com - a socially powered TV show and movie tracking service",
///     "loc": "New York, NY, USA",
///     "age": "30 years"
///   },
///   "account": {
///     "id": 51,
///     "timezone": "America/New_York",
///     "type": "free"
///   },
///   "connections": {
///     "facebook": true
///   }
/// }
/// ```
pub const SETTINGS_URL: &str = "https://api.simkl.com/users/settings";
pub const CHECKIN_URL: &str = "https://api.simkl.com/checkin";

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct User {
    pub name: String,
    joined_at: DateTime<Utc>,
    pub gender: String,
    /// Avatar URL
    pub avatar: String,
    pub bio: String,
    pub loc: String,
    pub age: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Account {
    pub id: u32,
    pub timezone: String,
    pub r#type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Connections {
    pub facebook: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Settings {
    pub user: User,
    pub account: Account,
    pub connections: Connections,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WatchedLastWeek {
    total_mins: u32,
    movies_mins: u32,
    tv_mins: u32,
    anime_mins: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Count {
    mins: u32,
    count: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Watched {
    pub watched_episodes_count: u16,
    pub count: u16,
    pub left_to_watch_episodes: u16,
    pub left_to_watch_mins: u16,
    pub total_episodes_count: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CompletedOrNotInteresting {
    pub watched_episodes_count: u16,
    pub count: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Movies {
    pub total_mins: u32,
    #[serde(rename = "plantowatch")]
    pub plan_to_watch: Count,
    #[serde(rename = "notinteresting")]
    pub not_interesting: Count,
    pub completed: CompletedOrNotInteresting,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TvOrAnime {
    total_mins: u32,
    pub watching: Watched,
    pub hold: Watched,
    #[serde(rename = "plantowatch")]
    pub plan_to_watch: Watched,
    #[serde(rename = "notinteresting")]
    pub not_interesting: CompletedOrNotInteresting,
    pub completed: CompletedOrNotInteresting,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stats {
    pub total_mins: u32,
    pub movies: Movies,
    pub tv: TvOrAnime,
    pub anime: TvOrAnime,
    pub watched_last_week: WatchedLastWeek,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LastWatchedArts {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub poster: String,
    pub fanart: String,
}

pub fn get_last_watched_arts(user_id: u32, client_id: String) -> String {
    let mut result = String::with_capacity(128);
    result.push_str(API_URL);
    result.push_str("users/recently-watched-background/");
    result.push_str(&user_id.to_string());
    result.push_str("?client_id=");
    result.push_str(&client_id);
    result
}

/// Response will redirect (`HTTP 302`) to the image to download or display. Header will look like:
///
/// `location: https://simkl.in/fanart/50/500671636445e211e_0.jpg`
pub fn get_last_watched_image_request(user_id: u32, fanart: bool, client_id: String) -> String {
    let mut result = String::with_capacity(128);
    result.push_str(API_URL);
    result.push_str("users/recently-watched-background/");
    result.push_str(&user_id.to_string());
    result.push_str("?image=");
    if fanart {
        result.push_str("fanart")
    } else {
        result.push_str("poster");
    }
    result.push_str("&client_id=");
    result.push_str(&client_id);
    result
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CheckinResponse {
    pub movie: Option<StandardMediaObject>,
    pub show: Option<StandardMediaObject>,
    pub anime: Option<StandardMediaObject>,
}

#[cfg(test)]
mod tests {
    use crate::user::get_last_watched_arts;

    #[test]
    fn test_last_watched_arts_request() {
        let client_id = String::from("azerty123456");
        assert_eq!(
            get_last_watched_arts(4321, client_id),
            "https://api.simkl.com/users/recently-watched-background/4321/?client_id=azerty123456"
        );
    }
}
