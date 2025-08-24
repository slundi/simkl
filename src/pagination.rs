use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    // pub offset: Option<u32>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
            // offset: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // pub fn offset(mut self, offset: u32) -> Self {
    //     self.offset = Some(offset);
    //     self
    // }

    pub fn to_query_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }

        // if let Some(offset) = self.offset {
        //     params.push(("offset", offset.to_string()));
        // }

        params
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Option<PaginationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub items_per_page: u32,
    pub has_next: bool,
    pub has_prev: bool,
}
