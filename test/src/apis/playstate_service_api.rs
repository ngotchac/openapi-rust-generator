use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct PlaystateServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PlaystateServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PlaystateServiceApiClient {
        PlaystateServiceApiClient {
            configuration,
        }
    }
}

impl PlaystateServiceApiClient {
    pub async fn delete_users_by_userid_playeditems_by_id(&self, user_id: &str, id: &str) -> Result<Box<crate::models::UserItemDataDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/PlayedItems/{Id}", configuration.base_path, UserId=crate::apis::urlencode(user_id), Id=crate::apis::urlencode(id));
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

    pub async fn delete_users_by_userid_playingitems_by_id(&self, user_id: &str, id: &str, media_source_id: &str, next_media_type: &str, position_ticks: Option<i64>, live_stream_id: Option<&str>, play_session_id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/PlayingItems/{Id}", configuration.base_path, UserId=crate::apis::urlencode(user_id), Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        req_builder = req_builder.query(&[("MediaSourceId", &media_source_id.to_string())]);
        req_builder = req_builder.query(&[("NextMediaType", &next_media_type.to_string())]);
        if let Some(ref s) = position_ticks {
            req_builder = req_builder.query(&[("PositionTicks", &s.to_string())]);
        }
        if let Some(ref s) = live_stream_id {
            req_builder = req_builder.query(&[("LiveStreamId", &s.to_string())]);
        }
        if let Some(ref s) = play_session_id {
            req_builder = req_builder.query(&[("PlaySessionId", &s.to_string())]);
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

    pub async fn post_sessions_playing(&self, playback_start_info: crate::models::PlaybackStartInfo) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Sessions/Playing", configuration.base_path);
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
        req_builder = req_builder.json(&playback_start_info);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_sessions_playing_ping(&self, play_session_id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Sessions/Playing/Ping", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = play_session_id {
            req_builder = req_builder.query(&[("PlaySessionId", &s.to_string())]);
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

    pub async fn post_sessions_playing_progress(&self, playback_progress_info: crate::models::PlaybackProgressInfo) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Sessions/Playing/Progress", configuration.base_path);
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
        req_builder = req_builder.json(&playback_progress_info);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_sessions_playing_stopped(&self, playback_stop_info: crate::models::PlaybackStopInfo) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Sessions/Playing/Stopped", configuration.base_path);
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
        req_builder = req_builder.json(&playback_stop_info);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_users_by_userid_playeditems_by_id(&self, user_id: &str, id: &str, date_played: Option<&str>) -> Result<Box<crate::models::UserItemDataDto>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/PlayedItems/{Id}", configuration.base_path, UserId=crate::apis::urlencode(user_id), Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = date_played {
            req_builder = req_builder.query(&[("DatePlayed", &s.to_string())]);
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

    pub async fn post_users_by_userid_playingitems_by_id(&self, user_id: &str, id: &str, media_source_id: &str, can_seek: Option<bool>, audio_stream_index: Option<i32>, subtitle_stream_index: Option<i32>, play_method: Option<&str>, live_stream_id: Option<&str>, play_session_id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/PlayingItems/{Id}", configuration.base_path, UserId=crate::apis::urlencode(user_id), Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("MediaSourceId", &media_source_id.to_string())]);
        if let Some(ref s) = can_seek {
            req_builder = req_builder.query(&[("CanSeek", &s.to_string())]);
        }
        if let Some(ref s) = audio_stream_index {
            req_builder = req_builder.query(&[("AudioStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = subtitle_stream_index {
            req_builder = req_builder.query(&[("SubtitleStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = play_method {
            req_builder = req_builder.query(&[("PlayMethod", &s.to_string())]);
        }
        if let Some(ref s) = live_stream_id {
            req_builder = req_builder.query(&[("LiveStreamId", &s.to_string())]);
        }
        if let Some(ref s) = play_session_id {
            req_builder = req_builder.query(&[("PlaySessionId", &s.to_string())]);
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

    pub async fn post_users_by_userid_playingitems_by_id_progress(&self, user_id: &str, id: &str, media_source_id: &str, position_ticks: Option<i64>, is_paused: Option<bool>, is_muted: Option<bool>, audio_stream_index: Option<i32>, subtitle_stream_index: Option<i32>, volume_level: Option<i32>, play_method: Option<&str>, live_stream_id: Option<&str>, play_session_id: Option<&str>, repeat_mode: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Users/{UserId}/PlayingItems/{Id}/Progress", configuration.base_path, UserId=crate::apis::urlencode(user_id), Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        req_builder = req_builder.query(&[("MediaSourceId", &media_source_id.to_string())]);
        if let Some(ref s) = position_ticks {
            req_builder = req_builder.query(&[("PositionTicks", &s.to_string())]);
        }
        if let Some(ref s) = is_paused {
            req_builder = req_builder.query(&[("IsPaused", &s.to_string())]);
        }
        if let Some(ref s) = is_muted {
            req_builder = req_builder.query(&[("IsMuted", &s.to_string())]);
        }
        if let Some(ref s) = audio_stream_index {
            req_builder = req_builder.query(&[("AudioStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = subtitle_stream_index {
            req_builder = req_builder.query(&[("SubtitleStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = volume_level {
            req_builder = req_builder.query(&[("VolumeLevel", &s.to_string())]);
        }
        if let Some(ref s) = play_method {
            req_builder = req_builder.query(&[("PlayMethod", &s.to_string())]);
        }
        if let Some(ref s) = live_stream_id {
            req_builder = req_builder.query(&[("LiveStreamId", &s.to_string())]);
        }
        if let Some(ref s) = play_session_id {
            req_builder = req_builder.query(&[("PlaySessionId", &s.to_string())]);
        }
        if let Some(ref s) = repeat_mode {
            req_builder = req_builder.query(&[("RepeatMode", &s.to_string())]);
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
