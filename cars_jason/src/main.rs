#[allow(dead_code)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

fn main() {
    let car = generated::Car {
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2022.0,
    };

    let wheel = generated::Wheel {
        radius: 17.5,
        width: 8.0,
    };

    println!("Car: {:?}", car);
    println!("Wheel: {:?}", wheel);
}