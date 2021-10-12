use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Typography {
    pub name: String,
    pub address: String,
    pub phone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub author_id: i32,
    pub name: String,
    pub category_id: i32,
    pub year: i32,
    pub type_id: i32,
    pub typography_id: i32,
    pub ordermaker_id: i32,
    pub price: f32
}
