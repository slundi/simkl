use serde::{Deserialize, Serialize};

use crate::{Episode, MediaType, API_URL};

pub trait SimklRequest {
    fn endpoint(&self) -> String;
    fn method(&self) -> &'static str {
        "GET"
    }
    fn query_params(&self) -> Vec<(String, String)> {
        Vec::new()
    }
    fn body(&self) -> Option<String> {
        None
    }
    fn headers(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn build_url(&self) -> String {
        let mut url = format!("{}{}", API_URL, self.endpoint());
        let params = self.query_params();
        if !params.is_empty() {
            url.push('?');
            let param_strings: Vec<String> = params
                .into_iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(&k), urlencoding::encode(&v)))
                .collect();
            url.push_str(&param_strings.join("&"));
        }
        url
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended: Option<String>,
}

impl SearchRequest {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            q: query.into(),
            r#type: None,
            limit: None,
            page: None,
            extended: None,
        }
    }

    pub fn with_type(mut self, media_type: MediaType) -> Self {
        self.r#type = Some(media_type);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.min(50)); // API max
        self
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_extended(mut self, extended: impl Into<String>) -> Self {
        self.extended = Some(extended.into());
        self
    }
}

impl SimklRequest for SearchRequest {
    fn endpoint(&self) -> String {
        match self.r#type {
            Some(MediaType::Movie) => "/search/movie".to_string(),
            Some(MediaType::Show) => "/search/tv".to_string(),
            Some(MediaType::Anime) => "/search/anime".to_string(),
            Some(MediaType::Episode) => "/search/episode".to_string(),
            None => "/search".to_string(),
        }
    }

    fn query_params(&self) -> Vec<(String, String)> {
        let mut params = vec![("q".to_string(), self.q.clone())];

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(ref extended) = self.extended {
            params.push(("extended".to_string(), extended.clone()));
        }

        params
    }
}

#[derive(Debug, Clone)]
pub struct MovieRequest {
    pub id: String,
    pub extended: Option<String>,
}

impl MovieRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            extended: None,
        }
    }

    pub fn with_extended(mut self, extended: impl Into<String>) -> Self {
        self.extended = Some(extended.into());
        self
    }
}

impl SimklRequest for MovieRequest {
    fn endpoint(&self) -> String {
        format!("/movies/{}", self.id)
    }

    fn query_params(&self) -> Vec<(String, String)> {
        if let Some(ref extended) = self.extended {
            vec![("extended".to_string(), extended.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShowRequest {
    pub id: String,
    pub extended: Option<String>,
}

impl ShowRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            extended: None,
        }
    }

    pub fn with_extended(mut self, extended: impl Into<String>) -> Self {
        self.extended = Some(extended.into());
        self
    }
}

impl SimklRequest for ShowRequest {
    fn endpoint(&self) -> String {
        format!("/tv/{}", self.id)
    }

    fn query_params(&self) -> Vec<(String, String)> {
        if let Some(ref extended) = self.extended {
            vec![("extended".to_string(), extended.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimeRequest {
    pub id: String,
    pub extended: Option<String>,
}

impl AnimeRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            extended: None,
        }
    }

    pub fn with_extended(mut self, extended: impl Into<String>) -> Self {
        self.extended = Some(extended.into());
        self
    }
}

impl SimklRequest for AnimeRequest {
    fn endpoint(&self) -> String {
        format!("/anime/{}", self.id)
    }

    fn query_params(&self) -> Vec<(String, String)> {
        if let Some(ref extended) = self.extended {
            vec![("extended".to_string(), extended.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct EpisodesRequest {
    pub show_id: String,
    pub season: Option<u32>,
    pub extended: Option<String>,
}

impl EpisodesRequest {
    pub fn new(show_id: impl Into<String>) -> Self {
        Self {
            show_id: show_id.into(),
            season: None,
            extended: None,
        }
    }

    pub fn with_season(mut self, season: u32) -> Self {
        self.season = Some(season);
        self
    }

    pub fn with_extended(mut self, extended: impl Into<String>) -> Self {
        self.extended = Some(extended.into());
        self
    }
}

impl SimklRequest for EpisodesRequest {
    fn endpoint(&self) -> String {
        if let Some(season) = self.season {
            format!("/tv/{}/episodes/{}", self.show_id, season)
        } else {
            format!("/tv/{}/episodes", self.show_id)
        }
    }

    fn query_params(&self) -> Vec<(String, String)> {
        if let Some(ref extended) = self.extended {
            vec![("extended".to_string(), extended.clone())]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EpisodesResponse {
    #[serde(default)]
    pub episodes: Vec<Episode>,
}
