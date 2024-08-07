use futures::{Stream, TryStreamExt};

use chrono::offset::Utc;
use chrono::DateTime;
use eventsource_client::{self as es, Client, Error};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    marker::PhantomData,
    time::SystemTime,
};
use urlencoding::encode;
use uuid::Uuid;

pub mod contants;

pub struct GlobeScraperClient<'a, T> {
    client: Box<dyn Client>,
    phantom: PhantomData<&'a T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryData {
    index: i8,
    r#type: String,
    searchbox_query: String,
    clicked_category: Option<String>,
    search_id: String,
    staged_image: Option<String>,
}

impl<'a, T> GlobeScraperClient<'a, T> {
    pub fn new(query: String) -> Result<Self, Error> {
        let user_id = Uuid::new_v4().to_string();
        let search_id = Uuid::new_v4().to_string();
        let model = String::from("default");
        let quer_data_object = vec![GlobeScraperClient::<T>::buil_query_data(
            query,
            search_id.clone(),
        )];
        let temp = serde_json::to_string(&quer_data_object).unwrap();
        let query_data = encode(&temp);
        let userid_local: String = GlobeScraperClient::<T>::build_user_id();

        let url = String::from(contants::GLOBE_BASE_URL)
            + &format!(
                "?queryData={}&userid_auth={}&model={}&search_id={}&userid_local={}",
                query_data, user_id, model, search_id, userid_local
            );

        let client = es::ClientBuilder::for_url(&url)?.build();

        Ok(GlobeScraperClient {
            client: Box::new(client),
            phantom: PhantomData,
        })
    }

    fn buil_query_data(search_query: String, search_id: String) -> QueryData {
        QueryData {
            index: 0,
            r#type: String::from("top_searchbox"),
            searchbox_query: search_query,
            clicked_category: None,
            search_id,
            staged_image: None,
        }
    }

    fn build_user_id() -> String {
        let mut user_prf = String::from("user");
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 9);
        let now = Into::<DateTime<Utc>>::into(SystemTime::now());

        user_prf.push_str(string.as_str());
        user_prf.push_str(now.timestamp().to_string().as_str());

        return String::from(user_prf);
    }

    pub fn get_page(
        &'a mut self,
        props: &'a mut Box<HashMap<String, String>>,
    ) -> impl Stream<Item = Result<(), ()>> + 'a {
        return self
            .client
            .stream()
            .map_ok(|event| match event {
                es::SSE::Event(ev) => {
                    let ev_data: Value = serde_json::from_str(&ev.data).unwrap();

                    match ev_data.get("type") {
                        Some(data) => {
                            let index = data.as_str().unwrap();

                            if index.to_owned() == String::from("top_answer_chunk") {
                                match ev_data.get("data") {
                                    Some(data) => {
                                        let formated = data
                                            .to_string()
                                            .as_str()
                                            .replace("\"", "")
                                            .replace("\n", "");

                                        if props.contains_key(contants::GLOBE_DESCRIPTION_KEY) {
                                            let mut desc = props
                                                .get(contants::GLOBE_DESCRIPTION_KEY)
                                                .unwrap()
                                                .clone();
                                            desc.push_str(formated.as_str());
                                            props.insert(
                                                contants::GLOBE_DESCRIPTION_KEY.to_owned(),
                                                desc,
                                            );
                                        } else {
                                            props.insert(
                                                contants::GLOBE_DESCRIPTION_KEY.to_owned(),
                                                formated,
                                            );
                                        }
                                    }
                                    None => todo!(),
                                }
                            }
                        }
                        None => todo!(),
                    }
                }
                es::SSE::Comment(_comment) => {}
            })
            .map_err(|err| {
                println!("error occurred : {:?}", err.to_string());
                ()
            });
    }
}