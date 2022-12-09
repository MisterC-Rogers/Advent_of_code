use std::env;
use core::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    
    let mut vector: Vec<i32> = Vec::new();
    let mut current_calories: i32 = 0;
    
    let my_file: File = File::open(file_path).unwrap();
    let reader = BufReader::new(my_file);
        
    for (_index, line) in reader.lines().enumerate() {
        let line: i32 = line.expect("Error reading line").parse().unwrap_or(0);
        
        if line == 0 {
            vector.push(current_calories);
            current_calories = 0;
        }
        current_calories+= line;
    }
    vector.push(current_calories);
    vector.sort_by_key(|w: &i32| Reverse(*w));
    let grand_total: i32 = vector[0] + vector[1] + vector[2];
    println!("{}, {}, {} equal {}", vector[0], vector[1], vector[2], grand_total)
}
// To Run 
// cargo run -- inputs.txt