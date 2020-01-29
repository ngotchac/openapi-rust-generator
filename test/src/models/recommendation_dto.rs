use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendationDto {
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::BaseItemDto>>,
    #[serde(rename(deserialize = "RecommendationType"))]
    pub recommendation_type: RecommendationType,
    #[serde(rename(deserialize = "BaselineItemName"))]
    pub baseline_item_name: String,
    #[serde(rename(deserialize = "CategoryId"))]
    pub category_id: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecommendationType {
    #[serde(rename = "SimilarToRecentlyPlayed")]
    SimilarToRecentlyPlayed,
    #[serde(rename = "SimilarToLikedItem")]
    SimilarToLikedItem,
    #[serde(rename = "HasDirectorFromRecentlyPlayed")]
    HasDirectorFromRecentlyPlayed,
    #[serde(rename = "HasActorFromRecentlyPlayed")]
    HasActorFromRecentlyPlayed,
    #[serde(rename = "HasLikedDirector")]
    HasLikedDirector,
    #[serde(rename = "HasLikedActor")]
    HasLikedActor,
}

