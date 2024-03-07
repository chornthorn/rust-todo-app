use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total_items: u32,
    pub total_pages: u32,
    pub page: u32,
    pub limit: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total_items: u32, page: u32, limit: u32) -> Self {
        let total_pages = if total_items % limit == 0 {
            total_items / limit
        } else {
            (total_items / limit) + 1
        };
        Self {
            items,
            total_items,
            total_pages,
            page,
            limit,
        }
    }

    // no pagination
    pub fn only_item(items: Vec<T>) -> Self {
        Self {
            items,
            total_items: 0,
            total_pages: 0,
            page: 0,
            limit: 0,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct PaginatedRequest {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,

    #[validate(length(min = 1, max = 100))]
    pub search: Option<String>,

    #[validate(length(min = 1, max = 10))]
    pub sort: Option<String>,
}

impl PaginatedRequest {
    pub fn new(page: Option<u32>, limit: Option<u32>, search: Option<String>, sort: Option<String>) -> Self {
        Self {
            page,
            limit,
            search,
            sort,
        }
    }

    pub fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
            search: None,
            sort: None,
        }
    }

    pub fn size_only(page: Option<u32>, limit: Option<u32>) -> Self {
        Self {
            page: if let Some(page) = page { Some(page) } else { Some(1) },
            limit: if let Some(limit) = limit { Some(limit) } else { Some(10) },
            search: None,
            sort: None,
        }
    }
}