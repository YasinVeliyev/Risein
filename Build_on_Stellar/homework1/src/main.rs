use homework1::customer::Customer;
use homework1::product::{self, Product};
use std::io;

fn main() {
    // let mut customer1 = Customer::new("Aygün", "Çoban", 200.5);
    // let mut customer2 = Customer::new("Pınar", "Topuz", 1200.5);
    // let mut customer3 = Customer::new("Dila", "Yapıcı", 52000.5);

    let product1 = Product::new("Diz Üstü Bilgisayar", 250., 52.);
    let product2 = Product::new("Iphone 16 pro", 2500., 520.);
    let product3 = Product::new("Dondurma", 2., 100.);
    let product4 = Product::new("Patates", 2., 500.);
    let products = &mut vec![product1, product2, product3, product4];

    let mut customer = Customer::default();
    let mut name = String::new();
    let mut surname = String::new();
    let mut balance = String::new();
    loop {
        if name.is_empty() {
            println!("Enter name");
            let _ = io::stdin().read_line(&mut name);
        }
        if surname.is_empty() {
            println!("Enter Surname");
            let _ = io::stdin().read_line(&mut surname);
        }
        if balance.is_empty() {
            println!("Enter Balance");
            let _ = io::stdin().read_line(&mut balance);
            match balance.trim().parse::<f32>() {
                Ok(num) => {
                    customer = Customer::new(&name, &surname, num);
                }
                Err(_) => {
                    println!("Enter a number");
                    balance.clear();
                    continue;
                }
            }
        }

        println!("Products");

        products
            .into_iter()
            .enumerate()
            .for_each(|(index, product)| {
                println!("{}:{:?}", index + 1, product);
            });

        let mut product_number = String::new();
        io::stdin().read_line(&mut product_number).expect("Error");

        match product_number.trim().parse::<usize>() {
            Ok(num) => {
                if let Some(product) = products.get_mut(num - 1) {
                    println!("Enter quantity");

                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Error");
                    let quantity = quantity.trim().parse::<f32>().unwrap();
                    if customer.buy_product(product, quantity) {
                        println!("You successfully purchased the product.");
                    } else {
                        println!("You couldn't purchase the product.");
                    }
                } else {
                    println!("Product does not exist");
                    continue;
                }
            }

            Err(_) => println!("Enter a number between 1-4"),
        }
    }
}
