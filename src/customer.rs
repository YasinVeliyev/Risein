use crate::product;

pub struct Customer {
    name: String,
    surname: String,
    balance: f32,
}

impl Customer {
    pub fn new(name: &str, surname: &str, balance: f32) -> Self {
        Self {
            name: name.to_string(),
            surname: surname.to_string(),
            balance,
        }
    }

    pub fn buy_product(&mut self, product: &mut product::Product, quantity: f32) -> bool {
        if product.quantity >= quantity && self.balance >= quantity * product.price {
            product.quantity -= quantity;
            self.balance -= quantity * product.price;
            true
        } else {
            false
        }
    }
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            name: String::new(),
            surname: String::new(),
            balance: 0.,
        }
    }
}
