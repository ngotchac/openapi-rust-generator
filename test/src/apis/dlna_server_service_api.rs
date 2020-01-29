use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct DlnaServerServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DlnaServerServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DlnaServerServiceApiClient {
        DlnaServerServiceApiClient {
            configuration,
        }
    }
}

impl DlnaServerServiceApiClient {
    pub async fn get_dlna_by_uuid_connectionmanager_connectionmanager(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/connectionmanager/connectionmanager", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_connectionmanager_connectionmanager_xml(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/connectionmanager/connectionmanager.xml", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_contentdirectory_contentdirectory(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/contentdirectory/contentdirectory", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_contentdirectory_contentdirectory_xml(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/contentdirectory/contentdirectory.xml", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_description(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/description", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_description_xml(&self, uu_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/description.xml", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_by_uuid_icons_by_filename(&self, uu_id: &str, filename: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/icons/{Filename}", configuration.base_path, UuId=crate::apis::urlencode(uu_id), Filename=crate::apis::urlencode(filename));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_dlna_icons_by_filename(&self, filename: &str, uu_id: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/icons/{Filename}", configuration.base_path, Filename=crate::apis::urlencode(filename));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = uu_id {
            req_builder = req_builder.query(&[("UuId", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_dlna_by_uuid_connectionmanager_control(&self, uu_id: &str, body: std::path::PathBuf) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/connectionmanager/control", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

    pub async fn post_dlna_by_uuid_contentdirectory_control(&self, uu_id: &str, body: std::path::PathBuf) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Dlna/{UuId}/contentdirectory/control", configuration.base_path, UuId=crate::apis::urlencode(uu_id));
        let mut req_builder = req_client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

		req_builder.send().await?;
        Ok(())
    }

}
