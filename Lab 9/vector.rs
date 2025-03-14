fn main() {
    let temperatures = vec![30, 32, 29, 35, 28, 31, 33];

    let average = calculate_average(&temperatures);
    let (highest, lowest) = find_high_low(&temperatures);

    println!("Average Temperature: {}", average);
    println!("Highest Temperature: {}", highest);
    println!("Lowest Temperature: {}", lowest);
}

fn calculate_average(temps: &Vec<i32>) -> f32 {
    let sum: i32 = temps.iter().sum();
    sum as f32 / temps.len() as f32
}

fn find_high_low(temps: &Vec<i32>) -> (i32, i32) {
    let highest = *temps.iter().max().unwrap();
    let lowest = *temps.iter().min().unwrap();
    (highest, lowest)
}

