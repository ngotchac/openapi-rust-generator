use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct InstantMixServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl InstantMixServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> InstantMixServiceApiClient {
        InstantMixServiceApiClient {
            configuration,
        }
    }
}

impl InstantMixServiceApiClient {
    pub async fn get_albums_by_id_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Albums/{Id}/InstantMix", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_artists_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Artists/InstantMix", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("Id", &id.to_string())]);
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_items_by_id_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/InstantMix", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_musicgenres_by_name_instantmix(&self, name: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/{Name}/InstantMix", configuration.base_path, Name=crate::apis::urlencode(name));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_musicgenres_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/InstantMix", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("Id", &id.to_string())]);
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_playlists_by_id_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Playlists/{Id}/InstantMix", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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

    pub async fn get_songs_by_id_instantmix(&self, id: &str, include_item_types: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, user_id: Option<&str>, limit: Option<i32>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Songs/{Id}/InstantMix", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
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
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
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
