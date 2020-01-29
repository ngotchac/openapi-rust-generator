use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct FilterServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl FilterServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> FilterServiceApiClient {
        FilterServiceApiClient {
            configuration,
        }
    }
}

impl FilterServiceApiClient {
    pub async fn get_items_filters(&self, user_id: Option<&str>, parent_id: Option<&str>, include_item_types: Option<&str>, media_types: Option<&str>) -> Result<Box<crate::models::QueryFiltersLegacy>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/Filters", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = parent_id {
            req_builder = req_builder.query(&[("ParentId", &s.to_string())]);
        }
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = media_types {
            req_builder = req_builder.query(&[("MediaTypes", &s.to_string())]);
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

    pub async fn get_items_filters2(&self, user_id: Option<&str>, parent_id: Option<&str>, include_item_types: Option<&str>, media_types: Option<&str>) -> Result<Box<crate::models::QueryFilters>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/Filters2", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = parent_id {
            req_builder = req_builder.query(&[("ParentId", &s.to_string())]);
        }
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = media_types {
            req_builder = req_builder.query(&[("MediaTypes", &s.to_string())]);
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
