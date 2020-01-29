use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct SearchServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SearchServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SearchServiceApiClient {
        SearchServiceApiClient {
            configuration,
        }
    }
}

impl SearchServiceApiClient {
    pub async fn get_search_hints(&self, search_term: &str, start_index: Option<i32>, limit: Option<i32>, user_id: Option<&str>, include_people: Option<bool>, include_media: Option<bool>, include_genres: Option<bool>, include_studios: Option<bool>, include_artists: Option<bool>, include_item_types: Option<&str>, exclude_item_types: Option<&str>, media_types: Option<&str>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>) -> Result<Box<crate::models::SearchSearchHintResult>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Search/Hints", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("SearchTerm", &search_term.to_string())]);
        if let Some(ref s) = include_people {
            req_builder = req_builder.query(&[("IncludePeople", &s.to_string())]);
        }
        if let Some(ref s) = include_media {
            req_builder = req_builder.query(&[("IncludeMedia", &s.to_string())]);
        }
        if let Some(ref s) = include_genres {
            req_builder = req_builder.query(&[("IncludeGenres", &s.to_string())]);
        }
        if let Some(ref s) = include_studios {
            req_builder = req_builder.query(&[("IncludeStudios", &s.to_string())]);
        }
        if let Some(ref s) = include_artists {
            req_builder = req_builder.query(&[("IncludeArtists", &s.to_string())]);
        }
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = exclude_item_types {
            req_builder = req_builder.query(&[("ExcludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = media_types {
            req_builder = req_builder.query(&[("MediaTypes", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

		let json_str = req_builder.send().await?.text().await?;

		let jd = &mut serde_json::Deserializer::from_str(&json_str);
		let data = match serde_path_to_error::deserialize(jd) {
			Ok(data) => data,
			Err(err) => {
				let path = err.path().to_string();
				eprintln!("Failed to parse JSON at {}", path);
				return Err(err.into());
			},
		};
		Ok(data)
    }

}
