use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct UniversalAudioServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl UniversalAudioServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> UniversalAudioServiceApiClient {
        UniversalAudioServiceApiClient {
            configuration,
        }
    }
}

impl UniversalAudioServiceApiClient {
    pub async fn get_audio_by_id_by_universal_container(&self, id: &str, container: &str, device_id: Option<&str>, start_time_ticks: Option<i64>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Audio/{Id}/universal.{Container}", configuration.base_path, Id=crate::apis::urlencode(id), Container=crate::apis::urlencode(container));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("DeviceId", &s.to_string())]);
        }
        if let Some(ref s) = start_time_ticks {
            req_builder = req_builder.query(&[("StartTimeTicks", &s.to_string())]);
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

    pub async fn get_audio_by_id_universal(&self, id: &str, device_id: Option<&str>, start_time_ticks: Option<i64>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Audio/{Id}/universal", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("DeviceId", &s.to_string())]);
        }
        if let Some(ref s) = start_time_ticks {
            req_builder = req_builder.query(&[("StartTimeTicks", &s.to_string())]);
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

    pub async fn head_audio_by_id_by_universal_container(&self, id: &str, container: &str, device_id: Option<&str>, start_time_ticks: Option<i64>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Audio/{Id}/universal.{Container}", configuration.base_path, Id=crate::apis::urlencode(id), Container=crate::apis::urlencode(container));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("DeviceId", &s.to_string())]);
        }
        if let Some(ref s) = start_time_ticks {
            req_builder = req_builder.query(&[("StartTimeTicks", &s.to_string())]);
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

    pub async fn head_audio_by_id_universal(&self, id: &str, device_id: Option<&str>, start_time_ticks: Option<i64>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Audio/{Id}/universal", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::HEAD, uri_str.as_str());

        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("DeviceId", &s.to_string())]);
        }
        if let Some(ref s) = start_time_ticks {
            req_builder = req_builder.query(&[("StartTimeTicks", &s.to_string())]);
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
