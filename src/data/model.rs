use serde::{Serialize, Deserialize};
use html_escape::encode_safe;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Typography {
    pub name: String,
    pub address: String,
    pub phone: String
}

impl Typography {
    pub fn escape(&mut self) {
        self.name = (*encode_safe(&mut self.name)).to_string();
        self.address = (*encode_safe(&self.address)).to_string();
        self.phone = (*encode_safe(&self.phone)).to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Order{
    pub fn escape(&mut self) {
        self.name = (*encode_safe(&mut self.name)).to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub name: String,
    pub birthday: String,
    pub zodiac_id: String
}

impl Author {
    pub fn escape(&mut self) {
        self.name = (*encode_safe(&mut self.name)).to_string();
        self.birthday = (*encode_safe(&mut self.birthday)).to_string();
        self.zodiac_id = (*encode_safe(&mut self.zodiac_id)).to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ordermaker {
    pub is_organization: bool,
    pub contact_name: String,
    pub address: String,
    pub phone: String,
    pub title: String
}

impl Ordermaker {
    pub fn escape(&mut self) {
        self.contact_name = (*encode_safe(&mut self.contact_name)).to_string();
        self.address = (*encode_safe(&mut self.address)).to_string();
        self.phone = (*encode_safe(&mut self.phone)).to_string();
        self.title = (*encode_safe(&mut self.title)).to_string();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action{
    pub id: i32,
    pub action: String,
    pub date: String
}

impl Action {
    pub fn escape(&mut self) {
        self.action = (*encode_safe(&mut self.action)).to_string();
        self.date = (*encode_safe(&mut self.date)).to_string();
    }
}
