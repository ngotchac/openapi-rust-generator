use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemPerson {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Role"))]
    pub role: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "PrimaryImageTag"))]
    pub primary_image_tag: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Actor")]
    Actor,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "Writer")]
    Writer,
    #[serde(rename = "Producer")]
    Producer,
    #[serde(rename = "GuestStar")]
    GuestStar,
    #[serde(rename = "Composer")]
    Composer,
    #[serde(rename = "Conductor")]
    Conductor,
    #[serde(rename = "Lyricist")]
    Lyricist,
}

