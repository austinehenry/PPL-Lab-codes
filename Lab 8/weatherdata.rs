fn calculate_average(temperatures: &[i32]) -> f64 {
    let sum: i32 = temperatures.iter().sum(); // Calculate the sum of temperatures in the slice
    let count = temperatures.len(); // Get the number of elements in the slice
    if count == 0 {
        0.0 // If the slice is empty, return 0.0 to avoid division by zero
    } else {
        sum as f64 / count as f64 // Calculate and return the average temperature as a float
    }
}

fn main() {
    // Array representing weekly temperature readings (in degrees Celsius)
    let temperatures = [22, 24, 21, 19, 18, 20, 23];

    // Extract the slice representing the last three days
    let last_three_days = &temperatures[4..];

    // Calculate the average temperature of the last three days
    let average_temperature = calculate_average(last_three_days);
    println!("Average temperature of the last three days: {:.2}°C", average_temperature);

    // Demonstrate safe handling of out-of-bounds access
    // Use .get() to safely attempt to access a slice
    let out_of_bounds_result = temperatures.get(10..15); // Safe slicing, won't panic if out of bounds

    match out_of_bounds_result {
        Some(slice) => println!("Valid slice: {:?}", slice),
        None => println!("Invalid slice, out of bounds."),
    }
}
