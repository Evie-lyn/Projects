fn make_sandwich (meat: &str, condiment: &str, bread: &str, count: i32){
    println! ("Making sandwich number {}: a {} and {} sandwich, on {}", count, meat, condiment, bread);
}

macro_rules! make_many_sandwiches {
    () => {
        {

            let mut sandwich_count = 0;
            let breads = ["sourdough", "rye", "baguette", "white bread", "whole wheat"];
            let meats = ["ham", "roast beef", "salami", "turkey", "chicken"];
            let condiments = ["mayo", "pesto", "ketchup", "honey mustard", "aioli"];

            for bread in breads.iter() {
                for meat in meats.iter(){
                    for condiment in condiments.iter() {
                        sandwich_count +=1;
                        make_sandwich (bread, meat, condiment, sandwich_count);
                    }
                }
            }
            println!("Total sandwiches made:{}", sandwich_count);
        }
    };
}
fn main() {
    make_many_sandwiches!();
}