use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct MoviesServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl MoviesServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> MoviesServiceApiClient {
        MoviesServiceApiClient {
            configuration,
        }
    }
}

impl MoviesServiceApiClient {
    pub async fn get_movies_recommendations(&self, category_limit: Option<i32>, item_limit: Option<i32>, user_id: Option<&str>, parent_id: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>) -> Result<Vec<Box<crate::models::RecommendationDto>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Movies/Recommendations", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = category_limit {
            req_builder = req_builder.query(&[("CategoryLimit", &s.to_string())]);
        }
        if let Some(ref s) = item_limit {
            req_builder = req_builder.query(&[("ItemLimit", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = parent_id {
            req_builder = req_builder.query(&[("ParentId", &s.to_string())]);
        }
        if let Some(ref s) = enable_images {
            req_builder = req_builder.query(&[("EnableImages", &s.to_string())]);
        }
        if let Some(ref s) = enable_user_data {
            req_builder = req_builder.query(&[("EnableUserData", &s.to_string())]);
        }
        if let Some(ref s) = image_type_limit {
            req_builder = req_builder.query(&[("ImageTypeLimit", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_types {
            req_builder = req_builder.query(&[("EnableImageTypes", &s.to_string())]);
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
