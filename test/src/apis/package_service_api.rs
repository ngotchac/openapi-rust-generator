use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct PackageServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PackageServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PackageServiceApiClient {
        PackageServiceApiClient {
            configuration,
        }
    }
}

impl PackageServiceApiClient {
    pub async fn delete_packages_installing_by_id(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Packages/Installing/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_packages(&self, package_type: Option<&str>, target_systems: Option<&str>, is_premium: Option<bool>, is_adult: Option<bool>) -> Result<Vec<Box<crate::models::UpdatesPackageInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Packages", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = package_type {
            req_builder = req_builder.query(&[("PackageType", &s.to_string())]);
        }
        if let Some(ref s) = target_systems {
            req_builder = req_builder.query(&[("TargetSystems", &s.to_string())]);
        }
        if let Some(ref s) = is_premium {
            req_builder = req_builder.query(&[("IsPremium", &s.to_string())]);
        }
        if let Some(ref s) = is_adult {
            req_builder = req_builder.query(&[("IsAdult", &s.to_string())]);
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

    pub async fn get_packages_by_name(&self, name: &str, assembly_guid: Option<&str>) -> Result<Box<crate::models::UpdatesPackageInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Packages/{Name}", configuration.base_path, Name=crate::apis::urlencode(name));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = assembly_guid {
            req_builder = req_builder.query(&[("AssemblyGuid", &s.to_string())]);
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

    pub async fn get_packages_updates(&self, package_type: &str) -> Result<Vec<Box<crate::models::UpdatesPackageVersionInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Packages/Updates", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("PackageType", &package_type.to_string())]);
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

    pub async fn post_packages_installed_by_name(&self, name: &str, assembly_guid: Option<&str>, version: Option<&str>, update_class: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Packages/Installed/{Name}", configuration.base_path, Name=crate::apis::urlencode(name));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = assembly_guid {
            req_builder = req_builder.query(&[("AssemblyGuid", &s.to_string())]);
        }
        if let Some(ref s) = version {
            req_builder = req_builder.query(&[("Version", &s.to_string())]);
        }
        if let Some(ref s) = update_class {
            req_builder = req_builder.query(&[("UpdateClass", &s.to_string())]);
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

		req_builder.send().await?;
        Ok(())
    }

}
