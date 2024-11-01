use serde::{Deserialize, Serialize};


// Existing data structures
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
pub struct Restaurant {
    pub restaurant: String,
    pub contact: String,
    pub menu: Menu,
    pub theme: Option<String>,
}