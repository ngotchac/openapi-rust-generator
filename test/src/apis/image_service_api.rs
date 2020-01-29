use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct ImageServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ImageServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ImageServiceApiClient {
        ImageServiceApiClient {
            configuration,
        }
    }
}

impl ImageServiceApiClient {
    pub async fn delete_items_by_id_images_by_type(&self, id: &str, _type: &str, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
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

    pub async fn delete_items_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
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

    pub async fn delete_users_by_id_images_by_type(&self, id: &str, _type: &str, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
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

    pub async fn delete_users_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
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

    pub async fn get_artists_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Artists/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_artists_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Artists/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_gamegenres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/GameGenres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_gamegenres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/GameGenres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_genres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Genres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_genres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Genres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_items_by_id_images(&self, id: &str) -> Result<Vec<Box<crate::models::ImageInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_items_by_id_images_by_type(&self, id: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_items_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_items_by_id_images_by_type_by_index_by_tag_by_format_by_maxwidth_by_maxheight_by_percentplayed_by_unplayedcount(&self, id: &str, max_width: Option<i32>, max_height: Option<i32>, tag: &str, format: &str, percent_played: Option<f64>, unplayed_count: Option<i32>, _type: &str, index: i32, width: Option<i32>, height: Option<i32>, quality: Option<i32>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, add_played_indicator: Option<bool>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}/{Tag}/{Format}/{MaxWidth}/{MaxHeight}/{PercentPlayed}/{UnplayedCount}", configuration.base_path, Id=crate::apis::urlencode(id), MaxWidth=max_width.unwrap(), MaxHeight=max_height.unwrap(), Tag=crate::apis::urlencode(tag), Format=crate::apis::urlencode(format), PercentPlayed=percent_played.unwrap(), UnplayedCount=unplayed_count.unwrap(), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_musicgenres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_musicgenres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_persons_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Persons/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_persons_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Persons/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_studios_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Studios/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_studios_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Studios/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_users_by_id_images_by_type(&self, id: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_users_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_artists_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Artists/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_artists_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Artists/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_gamegenres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/GameGenres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_gamegenres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/GameGenres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_genres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Genres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_genres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Genres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_items_by_id_images_by_type(&self, id: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_items_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_items_by_id_images_by_type_by_index_by_tag_by_format_by_maxwidth_by_maxheight_by_percentplayed_by_unplayedcount(&self, id: &str, max_width: Option<i32>, max_height: Option<i32>, tag: &str, format: &str, percent_played: Option<f64>, unplayed_count: Option<i32>, _type: &str, index: i32, width: Option<i32>, height: Option<i32>, quality: Option<i32>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, add_played_indicator: Option<bool>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}/{Tag}/{Format}/{MaxWidth}/{MaxHeight}/{PercentPlayed}/{UnplayedCount}", configuration.base_path, Id=crate::apis::urlencode(id), MaxWidth=max_width.unwrap(), MaxHeight=max_height.unwrap(), Tag=crate::apis::urlencode(tag), Format=crate::apis::urlencode(format), PercentPlayed=percent_played.unwrap(), UnplayedCount=unplayed_count.unwrap(), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_musicgenres_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_musicgenres_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/MusicGenres/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_persons_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Persons/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_persons_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Persons/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_studios_by_name_images_by_type(&self, name: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Studios/{Name}/Images/{Type}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_studios_by_name_images_by_type_by_index(&self, name: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Studios/{Name}/Images/{Type}/{Index}", configuration.base_path, Name=crate::apis::urlencode(name), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_users_by_id_images_by_type(&self, id: &str, _type: &str, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn head_users_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, max_width: Option<i32>, max_height: Option<i32>, width: Option<i32>, height: Option<i32>, quality: Option<i32>, tag: Option<&str>, crop_whitespace: Option<bool>, enable_image_enhancers: Option<bool>, format: Option<&str>, add_played_indicator: Option<bool>, percent_played: Option<f64>, unplayed_count: Option<i32>, background_color: Option<&str>, foreground_layer: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = quality {
            req_builder = req_builder.query(&[("Quality", &s.to_string())]);
        }
        if let Some(ref s) = tag {
            req_builder = req_builder.query(&[("Tag", &s.to_string())]);
        }
        if let Some(ref s) = crop_whitespace {
            req_builder = req_builder.query(&[("CropWhitespace", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_enhancers {
            req_builder = req_builder.query(&[("EnableImageEnhancers", &s.to_string())]);
        }
        if let Some(ref s) = format {
            req_builder = req_builder.query(&[("Format", &s.to_string())]);
        }
        if let Some(ref s) = add_played_indicator {
            req_builder = req_builder.query(&[("AddPlayedIndicator", &s.to_string())]);
        }
        if let Some(ref s) = percent_played {
            req_builder = req_builder.query(&[("PercentPlayed", &s.to_string())]);
        }
        if let Some(ref s) = unplayed_count {
            req_builder = req_builder.query(&[("UnplayedCount", &s.to_string())]);
        }
        if let Some(ref s) = background_color {
            req_builder = req_builder.query(&[("BackgroundColor", &s.to_string())]);
        }
        if let Some(ref s) = foreground_layer {
            req_builder = req_builder.query(&[("ForegroundLayer", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_items_by_id_images_by_type(&self, id: &str, _type: &str, body: std::path::PathBuf, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
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
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_items_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, body: std::path::PathBuf) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

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
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_items_by_id_images_by_type_by_index_index(&self, id: &str, _type: &str, index: i32, new_index: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Images/{Type}/{Index}/Index", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("NewIndex", &new_index.to_string())]);
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

    pub async fn post_users_by_id_images_by_type(&self, id: &str, _type: &str, body: std::path::PathBuf, index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = index {
            req_builder = req_builder.query(&[("Index", &s.to_string())]);
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
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_users_by_id_images_by_type_by_index(&self, id: &str, _type: &str, index: i32, body: std::path::PathBuf) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{Id}/Images/{Type}/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Type=crate::apis::urlencode(_type), Index=index);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

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
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

}
