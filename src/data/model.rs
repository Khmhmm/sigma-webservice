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

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub birthday: String,
    pub zodiac_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ordermaker {
    pub is_organization: bool,
    pub contact_name: String,
    pub address: String,
    pub phone: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action{
    pub id: i32,
    pub action: String,
    pub date: String
}
