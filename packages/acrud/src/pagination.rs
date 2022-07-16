use sea_orm::{DatabaseConnection, FromQueryResult, Paginator, SelectModel};
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub count: usize,
    pub total: usize,
    pub page: usize,
    pub page_count: usize,
}

pub async fn get_paginated_result<'db, T: FromQueryResult>(
    paginator: Paginator<'db, DatabaseConnection, SelectModel<T>>,
    data: Vec<T>,
) -> PaginatedResult<T> {
    let total = paginator.num_pages().await.ok().unwrap();
    let count = paginator.num_items().await.ok().unwrap();
    let page = paginator.cur_page();

    PaginatedResult {
        page,
        total,
        count,
        page_count: data.len(),
        data,
    }
}
