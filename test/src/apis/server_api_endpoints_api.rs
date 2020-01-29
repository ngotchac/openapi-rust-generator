use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct ServerApiEndpointsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ServerApiEndpointsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ServerApiEndpointsApiClient {
        ServerApiEndpointsApiClient {
            configuration,
        }
    }
}

impl ServerApiEndpointsApiClient {
    pub async fn post_notification_smtp_test_by_userid(&self, user_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Notification/SMTP/Test/{UserID}", configuration.base_path, UserID=crate::apis::urlencode(user_id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

}
