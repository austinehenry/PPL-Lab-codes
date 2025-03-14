// Define the FuelType enum
enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

// Define the Vehicle struct
struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

// Implement methods for Vehicle
impl Vehicle {
    // Constructor to create a new vehicle
    fn new(brand: &str, model: &str, fuel_type: FuelType) -> Vehicle {
        Vehicle {
            brand: brand.to_string(),
            model: model.to_string(),
            fuel_type,
        }
    }

    // Display vehicle details
    fn display(&self) {
        println!("Brand: {}, Model: {}", self.brand, self.model);
    }
}

// Function to filter and display electric vehicles
fn display_electric_vehicles(vehicles: &Vec<Vehicle>) {
    println!("Electric Vehicles:");
    for vehicle in vehicles {
        if let FuelType::Electric = vehicle.fuel_type {
            vehicle.display();
        }
    }
}

fn main() {
    // Create a list of vehicles
    let vehicles = vec![
        Vehicle::new("Tesla", "Model S", FuelType::Electric),
        Vehicle::new("Toyota", "Corolla", FuelType::Petrol),
        Vehicle::new("Nissan", "Leaf", FuelType::Electric),
        Vehicle::new("Ford", "F-150", FuelType::Diesel),
    ];

    // Display only electric vehicles
    display_electric_vehicles(&vehicles);
}
