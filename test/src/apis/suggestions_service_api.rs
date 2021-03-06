use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct SuggestionsServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SuggestionsServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SuggestionsServiceApiClient {
        SuggestionsServiceApiClient {
            configuration,
        }
    }
}

impl SuggestionsServiceApiClient {
    pub async fn get_users_by_userid_suggestions(&self, user_id: &str) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/Suggestions", configuration.base_path, UserId=crate::apis::urlencode(user_id));
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

}
