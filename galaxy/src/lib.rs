use serde::Deserialize;

const GALAXY: &str = include_str!("../galaxy.json");

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Access,
    #[serde(rename = "cargo storage")]
    CargoStorage,
    Collectible,
    Cosmetic,
    #[serde(rename = "crafting station")]
    CraftingStation,
    Crew,
    Currency,
    Emergence,
    Equipment,
    #[serde(rename = "landing pad")]
    LandingPad,
    Memories,
    Mining,
    Mud,
    Oni,
    Pack,
    Paint,
    Pet,
    Rebirth,
    Residential,
    Resource,
    Sage,
    Ship,
    Solarpunk,
    Story,
    Structure,
    Ustur,
    #[serde(rename = "yard item")]
    YardItem,
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
struct Nfts(Vec<Nft>);

#[derive(Debug, Deserialize)]
pub struct Nft {
    pub id: Option<String>,
    pub name: String,
    pub symbol: String,
    pub mint: String,
    pub attributes: Attributes,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub item_type: ItemType,
    pub category: Option<Category>,
}

/// A collection of Star Atlas NFTs.
#[derive(Debug)]
pub struct Galaxy {
    nfts: Nfts,
}

impl Galaxy {
    /// Creates a new `Galaxy` instance from the built-in `GALAXY` JSON data.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::Galaxy;
    ///
    /// let galaxy = Galaxy::new();
    /// ```
    pub fn new() -> Self {
        let nfts: Nfts = serde_json::from_str(GALAXY).expect("Failed to deserialize galaxy.json");
        Self { nfts }
    }

    /// Finds an NFT in the `Galaxy` by its mint ID.
    ///
    /// # Arguments
    ///
    /// * `mint` - The mint ID of the NFT to find.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::Galaxy;
    ///
    /// let galaxy = Galaxy::new();
    /// let nft = galaxy.find_mint("CPPRam7wKuBkYzN5zCffgNU17RKaeMEns4ZD83BqBVNR").unwrap();
    /// ```
    pub fn find_mint(&self, mint: &str) -> Option<&Nft> {
        self.nfts.0.iter().find(|nft| nft.mint == mint)
    }

    /// Finds an NFT in the `Galaxy` by its symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol of the NFT to find.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::Galaxy;
    ///
    /// let galaxy = Galaxy::new();
    /// let nft = galaxy.find_symbol("CUORE").unwrap();
    /// ```
    pub fn find_symbol(&self, symbol: &str) -> Option<&Nft> {
        self.nfts.0.iter().find(|nft| nft.symbol == symbol)
    }

    /// Returns a vector of all resource NFTs in the `Galaxy`.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::{Galaxy, ItemType, Category};
    ///
    /// let galaxy = Galaxy::new();
    /// let resources = galaxy.get_resources();
    /// ```
    pub fn get_resources(&self) -> Vec<&Nft> {
        self.filter_nft(ItemType::Resource, Some(Category::Resource))
    }

    /// Returns a vector of all ship NFTs in the `Galaxy`.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::{Galaxy, ItemType};
    ///
    /// let galaxy = Galaxy::new();
    /// let ships = galaxy.get_ships();
    /// ```
    pub fn get_ships(&self) -> Vec<&Nft> {
        self.filter_nft(ItemType::Ship, None)
    }

    /// Filters the NFTs in the `Galaxy` by item type and category.
    ///
    /// # Arguments
    ///
    /// * `item_type` - The item type to filter by.
    /// * `category` - The category to filter by, or `None` to ignore category.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_galaxy::{Galaxy, ItemType, Category};
    ///
    /// let galaxy = Galaxy::new();
    /// let resources = galaxy.filter_nft(ItemType::Resource, Some(Category::Resource));
    /// ```
    pub fn filter_nft(&self, item_type: ItemType, category: Option<Category>) -> Vec<&Nft> {
        self.nfts
            .0
            .iter()
            .filter(|nft| {
                if category.is_some() {
                    return nft.attributes.item_type == item_type
                        && nft.attributes.category == category;
                } else {
                    nft.attributes.item_type == item_type
                }
            })
            .collect()
    }
}

// cargo test -p staratlas-galaxy -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let galaxy = Galaxy::new();
        let ships = galaxy.get_ships();
        // dbg!(&ships);
        assert!(ships.len() > 0);

        let resources = galaxy.get_resources();
        // dbg!(&resources);
        assert!(resources.len() > 0);

        let copper_ore = galaxy.find_symbol("CUORE");
        // dbg!(&copper_ore);
        assert!(copper_ore.is_some());

        let copper = galaxy.find_mint("CPPRam7wKuBkYzN5zCffgNU17RKaeMEns4ZD83BqBVNR");
        // dbg!(&copper);
        assert!(copper.is_some());

        assert!(true);
    }
}
