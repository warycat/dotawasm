use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Data {
    pub objects: Vec<Table>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum Table {
    #[serde(alias = "alliances")]
    Alliances { rows: Vec<Alliance> },
    #[serde(alias = "attributes")]
    Attributes { rows: Vec<Attribute> },
    #[serde(alias = "heroes")]
    Heroes { rows: Vec<Hero> },
    #[serde(alias = "tiers")]
    Tiers { rows: Vec<Tier> },
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Alliance {
    pub id: String,
    pub name: String,
    pub modulo: i32,
    pub max: i32,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Attribute {
    pub hero_id: String,
    pub alliance_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Hero {
    pub id: String,
    pub name: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Tier {
    pub hero_id: String,
    pub tier: i32,
}
