use crate::errors::Error;
use reqwest::Url;
use serde::{Deserialize, Serialize};

const XIVAPI_URL: &str = "https://xivapi.com";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ItemsResponseMapping {
    results: Vec<Item>,
    pagination: PaginationMapping,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaginationMapping {
    page: u64,
    page_next: Option<u64>,
}

pub async fn get_all_items() -> Result<Vec<Item>, Error> {
    let mut url = Url::parse(XIVAPI_URL)?;
    url.path_segments_mut()
        .map_err(|_| Error::UrlParseBase)?
        .push("search");
    url.query_pairs_mut().append_pair("indexes", "Item");
    let mut page = 1;
    let mut results = vec![];

    loop {
        let mut url = url.clone();
        url.query_pairs_mut().append_pair("page", &page.to_string());

        let body_items = reqwest::get(url.as_str()).await?.text().await?;

        let mut items_page_response: ItemsResponseMapping = serde_json::from_str(&body_items)?;

        results.append(&mut items_page_response.results);

        page = match items_page_response.pagination.page_next {
            None => break,
            Some(page) => page,
        };
    }

    Ok(results)
}
