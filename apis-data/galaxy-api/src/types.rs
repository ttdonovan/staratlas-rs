use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Items(pub Vec<Item>);

#[derive(Debug, Deserialize)]
pub struct Item {
    pub _id: String,
    pub name: String,
    pub symbol: String,
    pub mint: String,
    pub attributes: Attributes,
    #[serde(default)]
    pub markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub item_type: ItemType,
    // pub category: Option<Category>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Access,
    Collectible,
    Crew,
    Currency,
    Memories,
    Resource,
    Ship,
    Story,
    Structure,
}

#[derive(Debug, Deserialize)]
pub struct Market {
    _id: Option<String>,
    id: String,
    quote_pair: Option<String>,
}