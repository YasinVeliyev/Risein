#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub quantity: f32,
}

impl Product {
    pub fn new(name: &str, price: f32, quantity: f32) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity,
        }
    }
}
