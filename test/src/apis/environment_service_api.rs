use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct EnvironmentServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl EnvironmentServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> EnvironmentServiceApiClient {
        EnvironmentServiceApiClient {
            configuration,
        }
    }
}

impl EnvironmentServiceApiClient {
    pub async fn get_environment_defaultdirectorybrowser(&self, ) -> Result<Box<crate::models::DefaultDirectoryBrowserInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/DefaultDirectoryBrowser", configuration.base_path);
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

    pub async fn get_environment_directorycontents(&self, path: &str, include_files: Option<bool>, include_directories: Option<bool>) -> Result<Vec<Box<crate::models::IoFileSystemEntryInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/DirectoryContents", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("Path", &path.to_string())]);
        if let Some(ref s) = include_files {
            req_builder = req_builder.query(&[("IncludeFiles", &s.to_string())]);
        }
        if let Some(ref s) = include_directories {
            req_builder = req_builder.query(&[("IncludeDirectories", &s.to_string())]);
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

    pub async fn get_environment_drives(&self, ) -> Result<Vec<Box<crate::models::IoFileSystemEntryInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/Drives", configuration.base_path);
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

    pub async fn get_environment_networkdevices(&self, ) -> Result<Vec<Box<crate::models::IoFileSystemEntryInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/NetworkDevices", configuration.base_path);
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

    pub async fn get_environment_networkshares(&self, path: &str) -> Result<Vec<Box<crate::models::IoFileSystemEntryInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/NetworkShares", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("Path", &path.to_string())]);
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

    pub async fn get_environment_parentpath(&self, path: &str) -> Result<String, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/ParentPath", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("Path", &path.to_string())]);
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

    pub async fn post_environment_validatepath(&self, path: &str, validate_path: crate::models::ValidatePath) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Environment/ValidatePath", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("Path", &path.to_string())]);
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
        req_builder = req_builder.json(&validate_path);

		req_builder.send().await?;
        Ok(())
    }

}
