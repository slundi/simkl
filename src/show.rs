use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShowStatus {
    Returning,
    Continuing,
    Ended,
    Canceled,
    Hiatus,
}

impl fmt::Display for ShowStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ShowStatus::Returning => "returning",
            ShowStatus::Continuing => "continuing",
            ShowStatus::Ended => "ended",
            ShowStatus::Canceled => "canceled",
            ShowStatus::Hiatus => "hiatus",
        };
        write!(f, "{}", s)
    }
}

#[allow(non_camel_case_types)]
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

// impl Show {
//     pub fn new(title: String) -> Self {
//         Self {
//             title,
//             year: None,
//             ids: None,
//             overview: None,
//             first_aired: None,
//             airs: None,
//             runtime: None,
//             certification: None,
//             network: None,
//             country: None,
//             language: None,
//             genres: None,
//             rating: None,
//             votes: None,
//             poster: None,
//             fanart: None,
//             status: None,
//             total_episodes: None,
//             aired_episodes: None,
//             anime_type: None,
//             watchers: None,
//             plays: None,
//             user_stats: None,
//         }
//     }

//     pub fn is_anime(&self) -> bool {
//         self.anime_type.is_some()
//     }

//     pub fn is_ongoing(&self) -> bool {
//         matches!(
//             self.status,
//             Some(ShowStatus::Returning | ShowStatus::Continuing)
//         )
//     }

//     pub fn completion_percentage(&self) -> Option<f32> {
//         match (self.aired_episodes, self.total_episodes) {
//             (Some(aired), Some(total)) if total > 0 => Some((aired as f32 / total as f32) * 100.0),
//             _ => None,
//         }
//     }
// }
