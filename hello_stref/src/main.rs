use rand::prelude::*;
use std::time::{Instant};

const AMOUNT_STRINGS: usize = 100000;
const STING_LENGTH: usize = 200;
const NEEDLE: &str = "needle";

fn main() {
    println! ("Generating {} strings of length {}!", AMOUNT_STRINGS, STING_LENGTH);
    let strings: Vec<String> = (0..AMOUNT_STRINGS)
        .map(|_| generate_random_string(STING_LENGTH))
        .collect();
    println!("Strings generated.");

    println!("\nRunning needle reference");
    let start_ref = Instant::now();
    for s in &strings {
        contains_needle_reference(s);
    }
    let duration_ref = start_ref.elapsed();
    println! ("Time for needle references: {:?}", duration_ref);

    println!("\nRunning needle ownership");
    let start_ownership = Instant::now();
    for s in strings.iter().cloned() {
        contains_needle_ownership(s);
    }
    let duration_ownership = start_ownership.elapsed();
    println! ("Time for needle ownership: {:?}", duration_ownership);

    let difference = duration_ownership - duration_ref;
    println! ("\nDifference in time: {:?}", difference);
}

fn generate_random_string(length: usize) -> String {
    const CHARD: &[u8] = b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";
    let mut rng = rand::thread_rng();
    let string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARD.len());
            CHARD[idx] as char
        })
        .collect();
    string
}

fn contains_needle_reference (haystack: &String) -> bool {
    haystack.contains(NEEDLE)
}
fn contains_needle_ownership (haystack: String) -> bool {
    haystack.contains(NEEDLE)
}