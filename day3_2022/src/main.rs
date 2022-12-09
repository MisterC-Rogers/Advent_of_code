use std::fs;
use std::collections::HashMap;
use iterchunks::IterChunks;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("part one => {}", part_1(&file));
    println!("part two => {}", part_2(&file))
}


fn part_1(input: &str) -> String {
    // make a mapping of the chars and value
    let letter_score = ('a'..='z').chain('A'..='Z').enumerate().map(|(index, letter)| (letter, index + 1)).collect::<HashMap<char, usize>>();
        
    let result: usize = input.lines().map(|line: &str|{
        // Get half way number
        let rucksack_length: usize = line.len() / 2;
        // get chars of compartment A
        let compartment_a: &str = &line[0..rucksack_length];
        // get chars of compartment B
        let compartment_b: &str = &line[rucksack_length..(rucksack_length * 2)];
        // find the common chars in both compartments
        let common_chars = compartment_a.chars().find(|c: &char| compartment_b.contains(*c)).unwrap();
        // get letter scores
        letter_score.get(&common_chars).unwrap()
    }).sum::<usize>();
    result.to_string()
}

fn part_2(input: &str) -> String {
    // make a mapping of the chars and value
    let letter_score = ('a'..='z').chain('A'..='Z').enumerate().map(|(index, letter)| (letter, index + 1)).collect::<HashMap<char, usize>>();
        
    let result: usize = input
        .lines().array_chunks::<3>()
        .map(|[a,b,c]|{
        let common_chars = a
        .chars()
        .find(|a_char|{
            b.contains(*a_char) && c.contains(*a_char)
        })
        .unwrap();
        letter_score.get(&common_chars).unwrap()
    }).sum::<usize>();
    result.to_string()
}