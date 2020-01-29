use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatesPackageInfo {
    #[serde(rename(deserialize = "id"))]
    pub id: String,
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "shortDescription"))]
    pub short_description: String,
    #[serde(rename(deserialize = "overview"))]
    pub overview: String,
    #[serde(rename(deserialize = "isPremium"))]
    pub is_premium: bool,
    #[serde(rename(deserialize = "adult"))]
    pub adult: bool,
    #[serde(rename(deserialize = "richDescUrl"))]
    pub rich_desc_url: String,
    #[serde(rename(deserialize = "thumbImage"))]
    pub thumb_image: String,
    #[serde(rename(deserialize = "previewImage"))]
    pub preview_image: String,
    #[serde(rename(deserialize = "type"))]
    pub _type: String,
    #[serde(rename(deserialize = "targetFilename"))]
    pub target_filename: String,
    #[serde(rename(deserialize = "owner"))]
    pub owner: String,
    #[serde(rename(deserialize = "category"))]
    pub category: String,
    #[serde(rename(deserialize = "tileColor"))]
    pub tile_color: String,
    #[serde(rename(deserialize = "featureId"))]
    pub feature_id: String,
    #[serde(rename(deserialize = "regInfo"))]
    pub reg_info: String,
    #[serde(rename(deserialize = "price"))]
    pub price: f32,
    #[serde(rename(deserialize = "targetSystem"))]
    pub target_system: TargetSystem,
    #[serde(rename(deserialize = "guid"))]
    pub guid: String,
    #[serde(rename(deserialize = "totalRatings"), skip_serializing_if = "Option::is_none")]
    pub total_ratings: Option<i32>,
    #[serde(rename(deserialize = "avgRating"))]
    pub avg_rating: f32,
    #[serde(rename(deserialize = "isRegistered"))]
    pub is_registered: bool,
    #[serde(rename(deserialize = "expDate"))]
    pub exp_date: String,
    #[serde(rename(deserialize = "versions"))]
    pub versions: Vec<Box<crate::models::UpdatesPackageVersionInfo>>,
    #[serde(rename(deserialize = "enableInAppStore"))]
    pub enable_in_app_store: bool,
    #[serde(rename(deserialize = "installs"))]
    pub installs: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetSystem {
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "MBTheater")]
    MBTheater,
    #[serde(rename = "MBClassic")]
    MBClassic,
}

