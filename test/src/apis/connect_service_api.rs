use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct ConnectServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ConnectServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ConnectServiceApiClient {
        ConnectServiceApiClient {
            configuration,
        }
    }
}

impl ConnectServiceApiClient {
    pub async fn delete_users_by_id_connect_link(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Connect/Link", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

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

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_connect_exchange(&self, connect_user_id: &str) -> Result<Box<crate::models::ConnectConnectAuthenticationExchangeResult>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Connect/Exchange", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("ConnectUserId", &connect_user_id.to_string())]);
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

    pub async fn get_connect_pending(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Connect/Pending", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

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

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_users_by_id_connect_link(&self, id: &str, connect_username: &str) -> Result<Box<crate::models::ConnectUserLinkResult>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Connect/Link", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("ConnectUsername", &connect_username.to_string())]);
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
