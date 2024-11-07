use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: u64,
    pub entries: Vec<Entries>
}

#[derive(Deserialize, Debug)]
pub struct Entries {
    pub product: Product,
    pub amount: u32,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub name: String,
    pub price: u32,
}