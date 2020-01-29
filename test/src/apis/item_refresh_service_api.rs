use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct ItemRefreshServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ItemRefreshServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ItemRefreshServiceApiClient {
        ItemRefreshServiceApiClient {
            configuration,
        }
    }
}

impl ItemRefreshServiceApiClient {
    pub async fn post_items_by_id_refresh(&self, id: &str, recursive: Option<bool>, metadata_refresh_mode: Option<&str>, image_refresh_mode: Option<&str>, replace_all_metadata: Option<bool>, replace_all_images: Option<bool>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Items/{Id}/Refresh", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = recursive {
            req_builder = req_builder.query(&[("Recursive", &s.to_string())]);
        }
        if let Some(ref s) = metadata_refresh_mode {
            req_builder = req_builder.query(&[("MetadataRefreshMode", &s.to_string())]);
        }
        if let Some(ref s) = image_refresh_mode {
            req_builder = req_builder.query(&[("ImageRefreshMode", &s.to_string())]);
        }
        if let Some(ref s) = replace_all_metadata {
            req_builder = req_builder.query(&[("ReplaceAllMetadata", &s.to_string())]);
        }
        if let Some(ref s) = replace_all_images {
            req_builder = req_builder.query(&[("ReplaceAllImages", &s.to_string())]);
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
