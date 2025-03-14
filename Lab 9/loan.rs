use std::io;

fn main() {
    let mut age_input = String::new();
    let mut income_input = String::new();

    println!("Enter your age:");
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a number");

    println!("Enter your income:");
    io::stdin().read_line(&mut income_input).expect("Failed to read line");
    let income: u32 = income_input.trim().parse().expect("Please enter a number");

    if age < 21 {
        println!("You are ineligible for a loan.");
    } else if age <= 60 {
        if income > 50000 {
            println!("You are eligible for a loan.");
        } else {
            println!("You are ineligible for a loan due to insufficient income.");
        }
    } else {
        println!("You need a guarantor for the loan.");
    }
}
