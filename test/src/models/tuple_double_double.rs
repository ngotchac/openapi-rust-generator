use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TupleDoubleDouble {
    #[serde(rename(deserialize = "Item1"))]
    pub item1: f64,
    #[serde(rename(deserialize = "Item2"))]
    pub item2: f64,
}


