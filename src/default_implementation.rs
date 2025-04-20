trait Vehicle {
    fn start(&self) -> bool;
    fn stop(&self) -> bool;

    fn status(&self) -> String {
        String::from("Vehicle is operational")
    }

    fn fuel_efficiency(&self) -> f64 {
        10.0 // Default fuel efficiency
        // mpg - this stand for miles per gallon
    }

    fn maintenance_check(&self) -> bool {
        true
    }
}

struct Car {
    make: String,
    model: String,
    year: u32,
    mpg: f64,
}

impl Vehicle for Car {
    fn start(&self) -> bool {
        println!("Starting the car: {} {}", self.make, self.model);
        true
    }

    fn stop(&self) -> bool {
        println!("Stopping the car: {} {}", self.make, self.model);
        true
    }

    fn status(&self) -> String {
        format!(
            "Car {} {} ({}): {} mpg",
            self.make, self.model, self.year, self.mpg
        )
    }

    fn fuel_efficiency(&self) -> f64 {
        self.mpg
    }
}

struct Bicycle {
    brand: String,
}

impl Vehicle for Bicycle {
    fn start(&self) -> bool {
        println!("Starting the bicycle: {}", self.brand);
        true
    }

    fn stop(&self) -> bool {
        println!("Stopping the bicycle: {}", self.brand);
        true
    }
}
