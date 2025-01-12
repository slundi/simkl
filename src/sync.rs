use std::result;

use crate::API_URL;

/// to retrieve the latest activity timestamps for the user. This endpoint provides timestamps for various categories
/// and media types, indicating the last time each was updated.
///
/// ```
/// POST https://api.simkl.com/sync/activities
// Headers:
///     Authorization: Bearer [token]
///     simkl-api-key: [client_id]
/// ```
pub const USER_ACTIVITIES: &str = "https://api.simkl.com/sync/activities";

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SyncSettings {
    pub all: DateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct MediaActivity {
    pub all: DateTime,
    pub rated_at: DateTime,
    #[serde(rename = "plantowatch")]
    pub plan_to_watch: DateTime,
    pub watching: Option<DateTime>,
    pub completed: DateTime,
    pub hold: Option<DateTime>,
    pub dropped: DateTime,
    pub removed_from_list: DateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Activities {
    pub all: DateTime,
    pub settings: SyncSettings,
    pub tv_shows: MediaActivity,
    pub anime: MediaActivity,
    pub movies: MediaActivity,
}

/// Retrieve the entire watchlist.
///
/// ```
/// POST https://api.simkl.com/sync/all-items/
// Headers:
///     Authorization: Bearer [token]
///     simkl-api-key: [client_id]
/// ```
pub const USER_ITEMS: &str = "https://api.simkl.com/sync/all-items/";

/// Instead of getting everything, you can get only one element (animes, movies, shows, ..., ratings, ...). You can
/// also use a starting date
pub fn get_all_items_request(
    what: String,
    from: Option<DateTime>,
    extended: Option<Extended>,
) -> String {
    // TODO: may use payload instead because we can filter on more stuffs: https://simkl.docs.apiary.io/reference/sync/get-all-items
    let mut result = String::from(USER_ITEMS);
    if let Some(w) = what {
        result.push_str(&w);
        result.push('/');
    }
    if let Some(d) = from {
        result.push_str("?date_from=");
        // TODO: from
    }
    result
}

pub fn get_add_to_history_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/history");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/add-items-to-the-history/add-items-to-watched/watching-history?console=1
    result
}

pub fn get_remove_from_history_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/history/remove");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/remove-items-from-history-and-from-lists/remove-items-from-watched/watching-history?console=1
    result
}

pub fn get_add_ratings_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/ratings");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/add-ratings/add-new-ratings?console=1
    result
}

pub fn get_remove_ratings_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/ratings/remove");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/remove-ratings/remove-ratings?console=1
    result
}

pub fn get_add_to_list_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/add-to-list");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/add-item-to-the-list/add-items-to-specific-list?console=1
    result
}

pub fn get_check_if_watched_request() -> String {
    let mut result = String::from(API_URL);
    result.push_str("sync/watched");
    // TODO: https://simkl.docs.apiary.io/#reference/sync/check-if-watched/get-specific-user's-watched-items?console=1
    result
}
