use serde:: {Deserialize, Serialize};
use serde_json::{Value, from_str, to_string_pretty};
use std::fs::File;
use std::io::Write;

#[derive (Deserialize, Serialize, Debug)]
struct Car {
    make: String,
    model: String,
}

fn process_cars () -> Result<Vec<Car>, Box<dyn std::error::Error>> {
    let json_str = include_str! ("../test.json");
    let json: Value = from_str(json_str)?;

    if let Some(cars_array) = json["cars"].as_array() {
        let cars: Result<Vec<Car>, _> = cars_array
            .iter()
            .map(|car_value| from_str(&car_value.to_string()))
            .collect();
        Ok (cars?)
    } else {
        Err ("Error: Not in the JSON.".into())
    }
}

fn main() -> Result<(), Box <dyn std::error::Error>> {
    let cars = process_cars ()?;

    println! ("Cars:");
    for car in &cars {
        println! ("Make: {}, Model: {}", car.make, car.model);
    }

    //Bonus
    let capitalize_cars: Vec<Car> = cars
        .into_iter()
        .map (|mut car| {
            car.make = car.make.to_uppercase();
            car.model = car.model.to_uppercase();
            car
        })
        .collect();
    let serialized_data = to_string_pretty(&capitalize_cars)?;

    let mut file = File::create("capitalized_cars.json")?;
    file.write_all(serialized_data.as_bytes())?;

    println! ("\nCapitalized Cars Written to capitalized_cars.json");

    Ok(())
}
