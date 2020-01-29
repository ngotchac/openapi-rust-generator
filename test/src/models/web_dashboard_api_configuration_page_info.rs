use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebDashboardApiConfigurationPageInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "EnableInMainMenu"))]
    pub enable_in_main_menu: bool,
    #[serde(rename(deserialize = "MenuSection"))]
    pub menu_section: String,
    #[serde(rename(deserialize = "MenuIcon"))]
    pub menu_icon: String,
    #[serde(rename(deserialize = "DisplayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "ConfigurationPageType"))]
    pub configuration_page_type: ConfigurationPageType,
    #[serde(rename(deserialize = "PluginId"))]
    pub plugin_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfigurationPageType {
    #[serde(rename = "PluginConfiguration")]
    PluginConfiguration,
    #[serde(rename = "None")]
    None,
}

