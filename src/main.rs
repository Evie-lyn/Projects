use serde_json::Value;

macro_rules! generate_foo {
    ($file_path:expr) => {
        fn foo() {
            let json_str = include_str! ("../test.json");
            let json: Value = serde_json::from_str(json_str).expect("Failed to read JSON");

            if let Some(data_array) = json ["data"].as_array() {
                for (index, value) in data_array.iter().enumerate() {
                    println! ("Num {} is {}", index, value);
                } 
            }
        }
    };
}
generate_foo! ("../test.json");

fn main() {
    foo();
}
