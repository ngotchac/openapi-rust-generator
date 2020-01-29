use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct UserViewsServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl UserViewsServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> UserViewsServiceApiClient {
        UserViewsServiceApiClient {
            configuration,
        }
    }
}

impl UserViewsServiceApiClient {
    pub async fn get_users_by_userid_groupingoptions(&self, user_id: &str) -> Result<Vec<Box<crate::models::BaseItemDto>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/GroupingOptions", configuration.base_path, UserId=crate::apis::urlencode(user_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

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

    pub async fn get_users_by_userid_views(&self, user_id: &str, include_external_content: Option<bool>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/Views", configuration.base_path, UserId=crate::apis::urlencode(user_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_external_content {
            req_builder = req_builder.query(&[("IncludeExternalContent", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

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
