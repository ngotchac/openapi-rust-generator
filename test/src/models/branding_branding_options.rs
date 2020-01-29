use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrandingBrandingOptions {
    #[serde(rename(deserialize = "LoginDisclaimer"))]
    pub login_disclaimer: String,
    #[serde(rename(deserialize = "CustomCss"))]
    pub custom_css: String,
}


