use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct NewsServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl NewsServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> NewsServiceApiClient {
        NewsServiceApiClient {
            configuration,
        }
    }
}

impl NewsServiceApiClient {
    pub async fn get_news_product(&self, start_index: Option<i32>, limit: Option<i32>) -> Result<Box<crate::models::QueryResultNewsNewsItem>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/News/Product", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
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
