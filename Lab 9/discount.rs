use std::io;

fn main() {
    let mut item_input = String::new();
    let mut quantity_input = String::new();

    println!("Enter menu item (Burger, Pizza, Pasta):");
    io::stdin().read_line(&mut item_input).expect("Failed to read line");
    let item = item_input.trim();

    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
    let quantity: u32 = quantity_input.trim().parse().expect("Please enter a number");

    let price = match item {
        "Burger" => 100,
        "Pizza" => 150,
        "Pasta" => 120,
        _ => {
            println!("Item not found.");
            return;
        }
    };

    let total_price = if quantity > 5 {
        (price * quantity) - ((price * quantity) * 10 / 100) // 10% discount
    } else {
        price * quantity
    };

    println!("Total price: ₹{}", total_price);
}

