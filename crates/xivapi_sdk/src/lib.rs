#![allow(non_snake_case)]

pub mod errors;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use crate::errors::Error;

const XIVAPI_URL: &str = "https://xivapi.com";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    #[serde(alias = "ID")]
    pub id: u64,
    #[serde(alias = "Name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ItemsResponseMapping {
    Results: Vec<Item>,
    Pagination: PaginationMapping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PaginationMapping {
    Page: u64,
    PageNext: Option<u64>,
}

pub async fn get_all_items() -> Result<Vec<Item>, Error> {
    let mut url = Url::parse(XIVAPI_URL)?;
    url.path_segments_mut()
        .map_err(|_| crate::errors::Error::UrlParseBase)?
        .push("search");
    url.query_pairs_mut().append_pair("indexes", "Item");

    let body_items = reqwest::get(url.as_str()).await?.text().await?;

    let mut items_response: ItemsResponseMapping = serde_json::from_str(&body_items)?;

    let mut page = match items_response.Pagination.PageNext {
        None => {
            return Ok(items_response.Results);
        }
        Some(page) => page,
    };

    loop {
        let mut url = Url::parse(XIVAPI_URL)?;
        url.path_segments_mut()
            .map_err(|_| crate::errors::Error::UrlParseBase)?
            .push("search");
        url.query_pairs_mut().append_pair("indexes", "Item");
        url.query_pairs_mut()
            .append_pair("page", &page.to_string()[..]);

        let body_items = reqwest::get(url.as_str()).await?.text().await?;

        let mut items_page_response: ItemsResponseMapping = serde_json::from_str(&body_items[..])?;

        items_response
            .Results
            .append(&mut items_page_response.Results);

        page = match items_page_response.Pagination.PageNext {
            None => break,
            Some(page) => page,
        };
    }

    Ok(items_response.Results)
}
