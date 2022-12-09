use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


// first column  second column     value
//      A               X            1
//      B               Y            2
//      C               Z            3

// win is 6 point
// draw is 3 point
// lost is 0 point

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    
    let my_file: File = File::open(file_path).unwrap();
    let reader = BufReader::new(my_file);
    
    let mut score: i32 = 0;
    
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let values: Vec<String> = line.split(" ").map(|s| s.to_string() ).collect();
        
        // part 1
        // if values[0] == "A" {
        //     match values[1].as_str() {
        //         "X" => score += 1 + 3,
        //         "Y" => score += 2 + 6,
        //         "Z" => score += 3 + 0,
        //         _ => panic!("Invalid play")
        //     }
        // } else if values[0] == "B" {
        //     match values[1].as_str() {
        //         "X" => score += 1 + 0,
        //         "Y" => score += 2 + 3,
        //         "Z" => score += 3 + 6,
        //         _ => panic!("Invalid play")
        //     }
        // } else if values[0] == "C" {
        //     match values[1].as_str() {
        //         "X" => score += 1 + 6,
        //         "Y" => score += 2 + 0,
        //         "Z" => score += 3 + 3,
        //         _ => panic!("Invalid play")
        //     }
        // }
        
        // part 2
        if values[0] == "A" {
            match values[1].as_str() {
                "X" => score += 3 + 0,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => panic!("Invalid play")
            }
        } else if values[0] == "B" {
            match values[1].as_str() {
                "X" => score += 1 + 0,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => panic!("Invalid play")
            }
        } else if values[0] == "C" {
            match values[1].as_str() {
                "X" => score += 2 + 0,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => panic!("Invalid play")
            }
        }
    }
    println!("{}", score)
}
// To Run 
// cargo run -- inputs.txt