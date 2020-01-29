use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct LiveTvServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl LiveTvServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> LiveTvServiceApiClient {
        LiveTvServiceApiClient {
            configuration,
        }
    }
}

impl LiveTvServiceApiClient {
    pub async fn delete_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn delete_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn delete_livetv_listingproviders(&self, id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("Id", &s.to_string())]);
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

    pub async fn delete_livetv_recordings_by_id(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn delete_livetv_seriestimers_by_id(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/SeriesTimers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn delete_livetv_timers_by_id(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn delete_livetv_tunerhosts(&self, id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/TunerHosts", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("Id", &s.to_string())]);
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

    pub async fn get_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn get_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn get_livetv_channels(&self, _type: Option<&str>, user_id: Option<&str>, start_index: Option<i32>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, limit: Option<i32>, is_favorite: Option<bool>, is_liked: Option<bool>, is_disliked: Option<bool>, enable_favorite_sorting: Option<bool>, enable_images: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, fields: Option<&str>, add_current_program: Option<bool>, enable_user_data: Option<bool>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Channels", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("Type", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = is_favorite {
            req_builder = req_builder.query(&[("IsFavorite", &s.to_string())]);
        }
        if let Some(ref s) = is_liked {
            req_builder = req_builder.query(&[("IsLiked", &s.to_string())]);
        }
        if let Some(ref s) = is_disliked {
            req_builder = req_builder.query(&[("IsDisliked", &s.to_string())]);
        }
        if let Some(ref s) = enable_favorite_sorting {
            req_builder = req_builder.query(&[("EnableFavoriteSorting", &s.to_string())]);
        }
        if let Some(ref s) = enable_images {
            req_builder = req_builder.query(&[("EnableImages", &s.to_string())]);
        }
        if let Some(ref s) = image_type_limit {
            req_builder = req_builder.query(&[("ImageTypeLimit", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_types {
            req_builder = req_builder.query(&[("EnableImageTypes", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
        }
        if let Some(ref s) = add_current_program {
            req_builder = req_builder.query(&[("AddCurrentProgram", &s.to_string())]);
        }
        if let Some(ref s) = enable_user_data {
            req_builder = req_builder.query(&[("EnableUserData", &s.to_string())]);
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

    pub async fn get_livetv_channels_by_id(&self, id: &str, user_id: Option<&str>) -> Result<Box<crate::models::BaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Channels/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
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

    pub async fn get_livetv_guideinfo(&self, ) -> Result<Box<crate::models::LiveTvGuideInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/GuideInfo", configuration.base_path);
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

    pub async fn get_livetv_info(&self, ) -> Result<Box<crate::models::LiveTvLiveTvInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Info", configuration.base_path);
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

    pub async fn get_livetv_listingproviders(&self, ) -> Result<Vec<Box<crate::models::LiveTvListingsProviderInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders", configuration.base_path);
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

    pub async fn get_livetv_listingproviders_available(&self, ) -> Result<Vec<Box<crate::models::LiveTvListingProviderTypeInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders/Available", configuration.base_path);
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

    pub async fn get_livetv_listingproviders_default(&self, ) -> Result<Box<crate::models::LiveTvListingsProviderInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders/Default", configuration.base_path);
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

    pub async fn get_livetv_listingproviders_lineups(&self, id: Option<&str>, _type: Option<&str>, location: Option<&str>, country: Option<&str>) -> Result<Vec<Box<crate::models::NameIdPair>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders/Lineups", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = id {
            req_builder = req_builder.query(&[("Id", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("Type", &s.to_string())]);
        }
        if let Some(ref s) = location {
            req_builder = req_builder.query(&[("Location", &s.to_string())]);
        }
        if let Some(ref s) = country {
            req_builder = req_builder.query(&[("Country", &s.to_string())]);
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

    pub async fn get_livetv_listingproviders_schedulesdirect_countries(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders/SchedulesDirect/Countries", configuration.base_path);
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

    pub async fn get_livetv_liverecordings_by_id_stream(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/LiveRecordings/{Id}/stream", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_livetv_livestreamfiles_by_id_by_container(&self, id: &str, container: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/LiveStreamFiles/{Id}/stream.{Container}", configuration.base_path, Id=crate::apis::urlencode(id), Container=crate::apis::urlencode(container));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_livetv_programs(&self, channel_ids: Option<&str>, user_id: Option<&str>, min_start_date: Option<&str>, has_aired: Option<bool>, max_start_date: Option<&str>, min_end_date: Option<&str>, max_end_date: Option<&str>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, start_index: Option<i32>, limit: Option<i32>, sort_by: Option<&str>, sort_order: Option<&str>, genre_ids: Option<&str>, enable_images: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, enable_user_data: Option<bool>, fields: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Programs", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = channel_ids {
            req_builder = req_builder.query(&[("ChannelIds", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = min_start_date {
            req_builder = req_builder.query(&[("MinStartDate", &s.to_string())]);
        }
        if let Some(ref s) = has_aired {
            req_builder = req_builder.query(&[("HasAired", &s.to_string())]);
        }
        if let Some(ref s) = max_start_date {
            req_builder = req_builder.query(&[("MaxStartDate", &s.to_string())]);
        }
        if let Some(ref s) = min_end_date {
            req_builder = req_builder.query(&[("MinEndDate", &s.to_string())]);
        }
        if let Some(ref s) = max_end_date {
            req_builder = req_builder.query(&[("MaxEndDate", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("SortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("SortOrder", &s.to_string())]);
        }
        if let Some(ref s) = genre_ids {
            req_builder = req_builder.query(&[("GenreIds", &s.to_string())]);
        }
        if let Some(ref s) = enable_images {
            req_builder = req_builder.query(&[("EnableImages", &s.to_string())]);
        }
        if let Some(ref s) = image_type_limit {
            req_builder = req_builder.query(&[("ImageTypeLimit", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_types {
            req_builder = req_builder.query(&[("EnableImageTypes", &s.to_string())]);
        }
        if let Some(ref s) = enable_user_data {
            req_builder = req_builder.query(&[("EnableUserData", &s.to_string())]);
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

    pub async fn get_livetv_programs_recommended(&self, user_id: Option<&str>, limit: Option<i32>, is_airing: Option<bool>, has_aired: Option<bool>, is_series: Option<bool>, is_movie: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, enable_images: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, genre_ids: Option<&str>, fields: Option<&str>, enable_user_data: Option<bool>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Programs/Recommended", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = is_airing {
            req_builder = req_builder.query(&[("IsAiring", &s.to_string())]);
        }
        if let Some(ref s) = has_aired {
            req_builder = req_builder.query(&[("HasAired", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref s) = enable_images {
            req_builder = req_builder.query(&[("EnableImages", &s.to_string())]);
        }
        if let Some(ref s) = image_type_limit {
            req_builder = req_builder.query(&[("ImageTypeLimit", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_types {
            req_builder = req_builder.query(&[("EnableImageTypes", &s.to_string())]);
        }
        if let Some(ref s) = genre_ids {
            req_builder = req_builder.query(&[("GenreIds", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
        }
        if let Some(ref s) = enable_user_data {
            req_builder = req_builder.query(&[("EnableUserData", &s.to_string())]);
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

    pub async fn get_livetv_recordings(&self, channel_id: Option<&str>, status: Option<&str>, is_in_progress: Option<bool>, series_timer_id: Option<&str>, artist_type: Option<&str>, max_official_rating: Option<&str>, has_theme_song: Option<bool>, has_theme_video: Option<bool>, has_subtitles: Option<bool>, has_special_feature: Option<bool>, has_trailer: Option<bool>, adjacent_to: Option<&str>, min_index_number: Option<i32>, min_players: Option<i32>, max_players: Option<i32>, parent_index_number: Option<i32>, has_parental_rating: Option<bool>, is_hd: Option<bool>, location_types: Option<&str>, exclude_location_types: Option<&str>, is_missing: Option<bool>, is_unaired: Option<bool>, min_community_rating: Option<f64>, min_critic_rating: Option<f64>, aired_during_season: Option<i32>, min_premiere_date: Option<&str>, min_date_last_saved: Option<&str>, min_date_last_saved_for_user: Option<&str>, max_premiere_date: Option<&str>, has_overview: Option<bool>, has_imdb_id: Option<bool>, has_tmdb_id: Option<bool>, has_tvdb_id: Option<bool>, exclude_item_ids: Option<&str>, start_index: Option<i32>, limit: Option<i32>, recursive: Option<bool>, sort_order: Option<&str>, parent_id: Option<&str>, fields: Option<&str>, exclude_item_types: Option<&str>, include_item_types: Option<&str>, any_provider_id_equals: Option<&str>, filters: Option<&str>, is_favorite: Option<bool>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, media_types: Option<&str>, image_types: Option<&str>, sort_by: Option<&str>, is_played: Option<bool>, genres: Option<&str>, official_ratings: Option<&str>, tags: Option<&str>, years: Option<&str>, enable_images: Option<bool>, enable_user_data: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, person: Option<&str>, person_ids: Option<&str>, person_types: Option<&str>, studios: Option<&str>, studio_ids: Option<&str>, artists: Option<&str>, artist_ids: Option<&str>, albums: Option<&str>, ids: Option<&str>, video_types: Option<&str>, containers: Option<&str>, audio_codecs: Option<&str>, video_codecs: Option<&str>, subtitle_codecs: Option<&str>, path: Option<&str>, user_id: Option<&str>, min_official_rating: Option<&str>, is_locked: Option<bool>, is_place_holder: Option<bool>, has_official_rating: Option<bool>, group_items_into_collections: Option<bool>, is3_d: Option<bool>, series_status: Option<&str>, name_starts_with_or_greater: Option<&str>, name_starts_with: Option<&str>, name_less_than: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = channel_id {
            req_builder = req_builder.query(&[("ChannelId", &s.to_string())]);
        }
        if let Some(ref s) = status {
            req_builder = req_builder.query(&[("Status", &s.to_string())]);
        }
        if let Some(ref s) = is_in_progress {
            req_builder = req_builder.query(&[("IsInProgress", &s.to_string())]);
        }
        if let Some(ref s) = series_timer_id {
            req_builder = req_builder.query(&[("SeriesTimerId", &s.to_string())]);
        }
        if let Some(ref s) = artist_type {
            req_builder = req_builder.query(&[("ArtistType", &s.to_string())]);
        }
        if let Some(ref s) = max_official_rating {
            req_builder = req_builder.query(&[("MaxOfficialRating", &s.to_string())]);
        }
        if let Some(ref s) = has_theme_song {
            req_builder = req_builder.query(&[("HasThemeSong", &s.to_string())]);
        }
        if let Some(ref s) = has_theme_video {
            req_builder = req_builder.query(&[("HasThemeVideo", &s.to_string())]);
        }
        if let Some(ref s) = has_subtitles {
            req_builder = req_builder.query(&[("HasSubtitles", &s.to_string())]);
        }
        if let Some(ref s) = has_special_feature {
            req_builder = req_builder.query(&[("HasSpecialFeature", &s.to_string())]);
        }
        if let Some(ref s) = has_trailer {
            req_builder = req_builder.query(&[("HasTrailer", &s.to_string())]);
        }
        if let Some(ref s) = adjacent_to {
            req_builder = req_builder.query(&[("AdjacentTo", &s.to_string())]);
        }
        if let Some(ref s) = min_index_number {
            req_builder = req_builder.query(&[("MinIndexNumber", &s.to_string())]);
        }
        if let Some(ref s) = min_players {
            req_builder = req_builder.query(&[("MinPlayers", &s.to_string())]);
        }
        if let Some(ref s) = max_players {
            req_builder = req_builder.query(&[("MaxPlayers", &s.to_string())]);
        }
        if let Some(ref s) = parent_index_number {
            req_builder = req_builder.query(&[("ParentIndexNumber", &s.to_string())]);
        }
        if let Some(ref s) = has_parental_rating {
            req_builder = req_builder.query(&[("HasParentalRating", &s.to_string())]);
        }
        if let Some(ref s) = is_hd {
            req_builder = req_builder.query(&[("IsHD", &s.to_string())]);
        }
        if let Some(ref s) = location_types {
            req_builder = req_builder.query(&[("LocationTypes", &s.to_string())]);
        }
        if let Some(ref s) = exclude_location_types {
            req_builder = req_builder.query(&[("ExcludeLocationTypes", &s.to_string())]);
        }
        if let Some(ref s) = is_missing {
            req_builder = req_builder.query(&[("IsMissing", &s.to_string())]);
        }
        if let Some(ref s) = is_unaired {
            req_builder = req_builder.query(&[("IsUnaired", &s.to_string())]);
        }
        if let Some(ref s) = min_community_rating {
            req_builder = req_builder.query(&[("MinCommunityRating", &s.to_string())]);
        }
        if let Some(ref s) = min_critic_rating {
            req_builder = req_builder.query(&[("MinCriticRating", &s.to_string())]);
        }
        if let Some(ref s) = aired_during_season {
            req_builder = req_builder.query(&[("AiredDuringSeason", &s.to_string())]);
        }
        if let Some(ref s) = min_premiere_date {
            req_builder = req_builder.query(&[("MinPremiereDate", &s.to_string())]);
        }
        if let Some(ref s) = min_date_last_saved {
            req_builder = req_builder.query(&[("MinDateLastSaved", &s.to_string())]);
        }
        if let Some(ref s) = min_date_last_saved_for_user {
            req_builder = req_builder.query(&[("MinDateLastSavedForUser", &s.to_string())]);
        }
        if let Some(ref s) = max_premiere_date {
            req_builder = req_builder.query(&[("MaxPremiereDate", &s.to_string())]);
        }
        if let Some(ref s) = has_overview {
            req_builder = req_builder.query(&[("HasOverview", &s.to_string())]);
        }
        if let Some(ref s) = has_imdb_id {
            req_builder = req_builder.query(&[("HasImdbId", &s.to_string())]);
        }
        if let Some(ref s) = has_tmdb_id {
            req_builder = req_builder.query(&[("HasTmdbId", &s.to_string())]);
        }
        if let Some(ref s) = has_tvdb_id {
            req_builder = req_builder.query(&[("HasTvdbId", &s.to_string())]);
        }
        if let Some(ref s) = exclude_item_ids {
            req_builder = req_builder.query(&[("ExcludeItemIds", &s.to_string())]);
        }
        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = recursive {
            req_builder = req_builder.query(&[("Recursive", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("SortOrder", &s.to_string())]);
        }
        if let Some(ref s) = parent_id {
            req_builder = req_builder.query(&[("ParentId", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
        }
        if let Some(ref s) = exclude_item_types {
            req_builder = req_builder.query(&[("ExcludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = include_item_types {
            req_builder = req_builder.query(&[("IncludeItemTypes", &s.to_string())]);
        }
        if let Some(ref s) = any_provider_id_equals {
            req_builder = req_builder.query(&[("AnyProviderIdEquals", &s.to_string())]);
        }
        if let Some(ref s) = filters {
            req_builder = req_builder.query(&[("Filters", &s.to_string())]);
        }
        if let Some(ref s) = is_favorite {
            req_builder = req_builder.query(&[("IsFavorite", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref s) = media_types {
            req_builder = req_builder.query(&[("MediaTypes", &s.to_string())]);
        }
        if let Some(ref s) = image_types {
            req_builder = req_builder.query(&[("ImageTypes", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("SortBy", &s.to_string())]);
        }
        if let Some(ref s) = is_played {
            req_builder = req_builder.query(&[("IsPlayed", &s.to_string())]);
        }
        if let Some(ref s) = genres {
            req_builder = req_builder.query(&[("Genres", &s.to_string())]);
        }
        if let Some(ref s) = official_ratings {
            req_builder = req_builder.query(&[("OfficialRatings", &s.to_string())]);
        }
        if let Some(ref s) = tags {
            req_builder = req_builder.query(&[("Tags", &s.to_string())]);
        }
        if let Some(ref s) = years {
            req_builder = req_builder.query(&[("Years", &s.to_string())]);
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
        if let Some(ref s) = person {
            req_builder = req_builder.query(&[("Person", &s.to_string())]);
        }
        if let Some(ref s) = person_ids {
            req_builder = req_builder.query(&[("PersonIds", &s.to_string())]);
        }
        if let Some(ref s) = person_types {
            req_builder = req_builder.query(&[("PersonTypes", &s.to_string())]);
        }
        if let Some(ref s) = studios {
            req_builder = req_builder.query(&[("Studios", &s.to_string())]);
        }
        if let Some(ref s) = studio_ids {
            req_builder = req_builder.query(&[("StudioIds", &s.to_string())]);
        }
        if let Some(ref s) = artists {
            req_builder = req_builder.query(&[("Artists", &s.to_string())]);
        }
        if let Some(ref s) = artist_ids {
            req_builder = req_builder.query(&[("ArtistIds", &s.to_string())]);
        }
        if let Some(ref s) = albums {
            req_builder = req_builder.query(&[("Albums", &s.to_string())]);
        }
        if let Some(ref s) = ids {
            req_builder = req_builder.query(&[("Ids", &s.to_string())]);
        }
        if let Some(ref s) = video_types {
            req_builder = req_builder.query(&[("VideoTypes", &s.to_string())]);
        }
        if let Some(ref s) = containers {
            req_builder = req_builder.query(&[("Containers", &s.to_string())]);
        }
        if let Some(ref s) = audio_codecs {
            req_builder = req_builder.query(&[("AudioCodecs", &s.to_string())]);
        }
        if let Some(ref s) = video_codecs {
            req_builder = req_builder.query(&[("VideoCodecs", &s.to_string())]);
        }
        if let Some(ref s) = subtitle_codecs {
            req_builder = req_builder.query(&[("SubtitleCodecs", &s.to_string())]);
        }
        if let Some(ref s) = path {
            req_builder = req_builder.query(&[("Path", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = min_official_rating {
            req_builder = req_builder.query(&[("MinOfficialRating", &s.to_string())]);
        }
        if let Some(ref s) = is_locked {
            req_builder = req_builder.query(&[("IsLocked", &s.to_string())]);
        }
        if let Some(ref s) = is_place_holder {
            req_builder = req_builder.query(&[("IsPlaceHolder", &s.to_string())]);
        }
        if let Some(ref s) = has_official_rating {
            req_builder = req_builder.query(&[("HasOfficialRating", &s.to_string())]);
        }
        if let Some(ref s) = group_items_into_collections {
            req_builder = req_builder.query(&[("GroupItemsIntoCollections", &s.to_string())]);
        }
        if let Some(ref s) = is3_d {
            req_builder = req_builder.query(&[("Is3D", &s.to_string())]);
        }
        if let Some(ref s) = series_status {
            req_builder = req_builder.query(&[("SeriesStatus", &s.to_string())]);
        }
        if let Some(ref s) = name_starts_with_or_greater {
            req_builder = req_builder.query(&[("NameStartsWithOrGreater", &s.to_string())]);
        }
        if let Some(ref s) = name_starts_with {
            req_builder = req_builder.query(&[("NameStartsWith", &s.to_string())]);
        }
        if let Some(ref s) = name_less_than {
            req_builder = req_builder.query(&[("NameLessThan", &s.to_string())]);
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

    pub async fn get_livetv_recordings_by_id(&self, id: &str, user_id: Option<&str>) -> Result<Box<crate::models::BaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
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

    pub async fn get_livetv_recordings_folders(&self, user_id: Option<&str>) -> Result<Vec<Box<crate::models::BaseItemDto>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/Folders", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
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

    pub async fn get_livetv_recordings_groups(&self, user_id: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/Groups", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
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

    pub async fn get_livetv_recordings_groups_by_id(&self, id: &str) -> Result<Box<crate::models::BaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/Groups/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_livetv_recordings_series(&self, channel_id: Option<&str>, user_id: Option<&str>, group_id: Option<&str>, start_index: Option<i32>, limit: Option<i32>, status: Option<&str>, is_in_progress: Option<bool>, series_timer_id: Option<&str>, enable_images: Option<bool>, image_type_limit: Option<i32>, enable_image_types: Option<&str>, fields: Option<&str>, enable_user_data: Option<bool>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Recordings/Series", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = channel_id {
            req_builder = req_builder.query(&[("ChannelId", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = group_id {
            req_builder = req_builder.query(&[("GroupId", &s.to_string())]);
        }
        if let Some(ref s) = start_index {
            req_builder = req_builder.query(&[("StartIndex", &s.to_string())]);
        }
        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("Limit", &s.to_string())]);
        }
        if let Some(ref s) = status {
            req_builder = req_builder.query(&[("Status", &s.to_string())]);
        }
        if let Some(ref s) = is_in_progress {
            req_builder = req_builder.query(&[("IsInProgress", &s.to_string())]);
        }
        if let Some(ref s) = series_timer_id {
            req_builder = req_builder.query(&[("SeriesTimerId", &s.to_string())]);
        }
        if let Some(ref s) = enable_images {
            req_builder = req_builder.query(&[("EnableImages", &s.to_string())]);
        }
        if let Some(ref s) = image_type_limit {
            req_builder = req_builder.query(&[("ImageTypeLimit", &s.to_string())]);
        }
        if let Some(ref s) = enable_image_types {
            req_builder = req_builder.query(&[("EnableImageTypes", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[("Fields", &s.to_string())]);
        }
        if let Some(ref s) = enable_user_data {
            req_builder = req_builder.query(&[("EnableUserData", &s.to_string())]);
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

    pub async fn get_livetv_seriestimers(&self, sort_by: Option<&str>, sort_order: Option<&str>) -> Result<Box<crate::models::QueryResultLiveTvSeriesTimerInfoDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/SeriesTimers", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("SortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("SortOrder", &s.to_string())]);
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

    pub async fn get_livetv_seriestimers_by_id(&self, id: &str) -> Result<Box<crate::models::LiveTvTimerInfoDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/SeriesTimers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_livetv_timers(&self, channel_id: Option<&str>, series_timer_id: Option<&str>) -> Result<Box<crate::models::QueryResultLiveTvTimerInfoDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = channel_id {
            req_builder = req_builder.query(&[("ChannelId", &s.to_string())]);
        }
        if let Some(ref s) = series_timer_id {
            req_builder = req_builder.query(&[("SeriesTimerId", &s.to_string())]);
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

    pub async fn get_livetv_timers_by_id(&self, id: &str) -> Result<Box<crate::models::LiveTvTimerInfoDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_livetv_timers_defaults(&self, program_id: Option<&str>) -> Result<Box<crate::models::LiveTvSeriesTimerInfoDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers/Defaults", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = program_id {
            req_builder = req_builder.query(&[("ProgramId", &s.to_string())]);
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

    pub async fn get_livetv_tunerhosts(&self, ) -> Result<Vec<Box<crate::models::LiveTvTunerHostInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/TunerHosts", configuration.base_path);
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

    pub async fn get_livetv_tunerhosts_types(&self, ) -> Result<Vec<Box<crate::models::NameIdPair>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/TunerHosts/Types", configuration.base_path);
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

    pub async fn get_livetv_tuners_discvover(&self, ) -> Result<Vec<Box<crate::models::LiveTvTunerHostInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Tuners/Discvover", configuration.base_path);
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

    pub async fn head_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn head_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn options_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::OPTIONS, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn options_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::OPTIONS, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn patch_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::PATCH, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn patch_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::PATCH, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn post_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn post_livetv_channelmappings(&self, provider_id: &str, live_tv_set_channel_mapping: crate::models::LiveTvSetChannelMapping) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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
        req_builder = req_builder.json(&live_tv_set_channel_mapping);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_livetv_listingproviders(&self, live_tv_listings_provider_info: crate::models::LiveTvListingsProviderInfo) -> Result<Box<crate::models::LiveTvListingsProviderInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ListingProviders", configuration.base_path);
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
        req_builder = req_builder.json(&live_tv_listings_provider_info);

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

    pub async fn post_livetv_programs(&self, live_tv_get_programs: crate::models::LiveTvGetPrograms, channel_ids: Option<&str>, user_id: Option<&str>, min_start_date: Option<&str>, max_start_date: Option<&str>, min_end_date: Option<&str>, max_end_date: Option<&str>, is_movie: Option<bool>, is_series: Option<bool>, is_news: Option<bool>, is_kids: Option<bool>, is_sports: Option<bool>, genre_ids: Option<&str>) -> Result<Box<crate::models::QueryResultBaseItemDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Programs", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = channel_ids {
            req_builder = req_builder.query(&[("ChannelIds", &s.to_string())]);
        }
        if let Some(ref s) = user_id {
            req_builder = req_builder.query(&[("UserId", &s.to_string())]);
        }
        if let Some(ref s) = min_start_date {
            req_builder = req_builder.query(&[("MinStartDate", &s.to_string())]);
        }
        if let Some(ref s) = max_start_date {
            req_builder = req_builder.query(&[("MaxStartDate", &s.to_string())]);
        }
        if let Some(ref s) = min_end_date {
            req_builder = req_builder.query(&[("MinEndDate", &s.to_string())]);
        }
        if let Some(ref s) = max_end_date {
            req_builder = req_builder.query(&[("MaxEndDate", &s.to_string())]);
        }
        if let Some(ref s) = is_movie {
            req_builder = req_builder.query(&[("IsMovie", &s.to_string())]);
        }
        if let Some(ref s) = is_series {
            req_builder = req_builder.query(&[("IsSeries", &s.to_string())]);
        }
        if let Some(ref s) = is_news {
            req_builder = req_builder.query(&[("IsNews", &s.to_string())]);
        }
        if let Some(ref s) = is_kids {
            req_builder = req_builder.query(&[("IsKids", &s.to_string())]);
        }
        if let Some(ref s) = is_sports {
            req_builder = req_builder.query(&[("IsSports", &s.to_string())]);
        }
        if let Some(ref s) = genre_ids {
            req_builder = req_builder.query(&[("GenreIds", &s.to_string())]);
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
        req_builder = req_builder.json(&live_tv_get_programs);

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

    pub async fn post_livetv_seriestimers(&self, live_tv_series_timer_info_dto: crate::models::LiveTvSeriesTimerInfoDto) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/SeriesTimers", configuration.base_path);
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
        req_builder = req_builder.json(&live_tv_series_timer_info_dto);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_livetv_seriestimers_by_id(&self, id: &str, live_tv_series_timer_info_dto: crate::models::LiveTvSeriesTimerInfoDto) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/SeriesTimers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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
        req_builder = req_builder.json(&live_tv_series_timer_info_dto);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_livetv_timers(&self, live_tv_timer_info_dto: crate::models::LiveTvTimerInfoDto) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers", configuration.base_path);
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
        req_builder = req_builder.json(&live_tv_timer_info_dto);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_livetv_timers_by_id(&self, id: &str, live_tv_timer_info_dto: crate::models::LiveTvTimerInfoDto) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Timers/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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
        req_builder = req_builder.json(&live_tv_timer_info_dto);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_livetv_tunerhosts(&self, live_tv_tuner_host_info: crate::models::LiveTvTunerHostInfo) -> Result<Box<crate::models::LiveTvTunerHostInfo>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/TunerHosts", configuration.base_path);
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
        req_builder = req_builder.json(&live_tv_tuner_host_info);

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

    pub async fn post_livetv_tuners_by_id_reset(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/Tuners/{Id}/Reset", configuration.base_path, Id=crate::apis::urlencode(id));
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

		req_builder.send().await?;
        Ok(())
    }

    pub async fn put_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::PUT, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn put_livetv_channelmappings(&self, provider_id: &str, live_tv_set_channel_mapping: crate::models::LiveTvSetChannelMapping) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::PUT, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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
        req_builder = req_builder.json(&live_tv_set_channel_mapping);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn trace_livetv_channelmappingoptions(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappingOptions", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::TRACE, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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

    pub async fn trace_livetv_channelmappings(&self, provider_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/LiveTv/ChannelMappings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::TRACE, uri_str.as_str());

        req_builder = req_builder.query(&[("ProviderId", &provider_id.to_string())]);
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
