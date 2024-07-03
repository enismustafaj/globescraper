use std::collections::HashMap;

use client::GlobeScraperClient;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use globescraper::client;

#[derive(Serialize, Deserialize, Debug)]
struct QueryData {
    index: i8,
    r#type: String,
    searchbox_query: String,
    clicked_category: Option<String>,
    search_id: String,
    staged_image: Option<String>,
}

#[tokio::main]
async fn main() -> () {
    let globe_client = GlobeScraperClient::<String>::new(String::from("bat ctle"), String::from("description"));

    let mut props = Box::new(HashMap::<String, String>::new());

    match globe_client {
        Ok(mut client) => {
            let mut stream = client.get_page(&mut props);

            while let Ok(Some(_)) = stream.try_next().await {}
        }
        Err(_) => todo!(),
    }

    print!("{}", props.get("description").unwrap());

}
