use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct SubtitleServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SubtitleServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SubtitleServiceApiClient {
        SubtitleServiceApiClient {
            configuration,
        }
    }
}

impl SubtitleServiceApiClient {
    pub async fn delete_videos_by_id_subtitles_by_index(&self, id: &str, index: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/{Id}/Subtitles/{Index}", configuration.base_path, Id=crate::apis::urlencode(id), Index=index);
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

    pub async fn get_items_by_id_remotesearch_subtitles_by_language(&self, id: &str, language: &str, is_perfect_match: Option<bool>, is_forced: Option<bool>) -> Result<Vec<Box<crate::models::RemoteSubtitleInfo>>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/RemoteSearch/Subtitles/{Language}", configuration.base_path, Id=crate::apis::urlencode(id), Language=crate::apis::urlencode(language));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = is_perfect_match {
            req_builder = req_builder.query(&[("IsPerfectMatch", &s.to_string())]);
        }
        if let Some(ref s) = is_forced {
            req_builder = req_builder.query(&[("IsForced", &s.to_string())]);
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

    pub async fn get_providers_subtitles_subtitles_by_id(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Providers/Subtitles/Subtitles/{Id}", configuration.base_path, Id=crate::apis::urlencode(id));
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

    pub async fn get_videos_by_id_by_mediasourceid_subtitles_by_index_by_format(&self, id: &str, media_source_id: &str, index: i32, format: &str, start_position_ticks: Option<i64>, end_position_ticks: Option<i64>, copy_timestamps: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/{Id}/{MediaSourceId}/Subtitles/{Index}/Stream.{Format}", configuration.base_path, Id=crate::apis::urlencode(id), MediaSourceId=crate::apis::urlencode(media_source_id), Index=index, Format=crate::apis::urlencode(format));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start_position_ticks {
            req_builder = req_builder.query(&[("StartPositionTicks", &s.to_string())]);
        }
        if let Some(ref s) = end_position_ticks {
            req_builder = req_builder.query(&[("EndPositionTicks", &s.to_string())]);
        }
        if let Some(ref s) = copy_timestamps {
            req_builder = req_builder.query(&[("CopyTimestamps", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_videos_by_id_by_mediasourceid_subtitles_by_index_by_startpositionticks_by_format(&self, id: &str, media_source_id: &str, index: i32, format: &str, start_position_ticks: i64, end_position_ticks: Option<i64>, copy_timestamps: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/{Id}/{MediaSourceId}/Subtitles/{Index}/{StartPositionTicks}/Stream.{Format}", configuration.base_path, Id=crate::apis::urlencode(id), MediaSourceId=crate::apis::urlencode(media_source_id), Index=index, Format=crate::apis::urlencode(format), StartPositionTicks=start_position_ticks);
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = end_position_ticks {
            req_builder = req_builder.query(&[("EndPositionTicks", &s.to_string())]);
        }
        if let Some(ref s) = copy_timestamps {
            req_builder = req_builder.query(&[("CopyTimestamps", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_items_by_id_remotesearch_subtitles_by_subtitleid(&self, id: &str, subtitle_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/RemoteSearch/Subtitles/{SubtitleId}", configuration.base_path, Id=crate::apis::urlencode(id), SubtitleId=crate::apis::urlencode(subtitle_id));
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

}
