use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub per_page: Option<i64> 
}

impl Pagination {
    pub fn limit_offset(&self) -> (i64, i64) {
        let per_page = self.per_page.unwrap_or(50).clamp(1, 100);
        let page = self.page.unwrap_or(1).max(1);

        let offset = (page - 1) * per_page;

        (per_page, offset)
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}