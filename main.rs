use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


fn main() {
    println!("Please enter the file path:"); //read file path from user
    
    let mut filepath = String::new();
   
    io::stdin()
            .read_line(&mut filepath)
            .expect("Failed to read line");
    let filepath = filepath.trim();
       
    match count_characters(filepath) {
        Ok(counts) => {
            println!("\nCharacter Counts:"); //prints occurances
            for (char, count) in counts{
                println!("'{}': {}", char, count);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }

    }
}

//stores in string variable
fn count_characters (filepath: &str) -> io::Result<HashMap<char, u32>> {
    let mut file = File::open(filepath)?; 

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    //counts characters
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for char in text.chars() {
        *char_counts.entry(char).or_insert(0) += 1;
    }

    Ok(char_counts)
}