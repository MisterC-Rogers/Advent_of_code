use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part_1(&file));
    println!("{}", part_2(&file));
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}


fn part_1(input: &str) -> usize {
    let mut pointer_a = 0;
    let mut pointer_b = 4;
    let mut soc = 0;
    for c in input.chars().enumerate() {
        let substring = &input[pointer_a..=pointer_b];
        match unique(substring) {
            None => {
                println!("is unique at {}", pointer_b);
                soc = pointer_b;
                break;
            },
            Some((i, j, c)) => println!(
                "is not unique \"{}\" at indices {} and {}",
                c, i, j
            ),
        }
        pointer_a += 1;
        pointer_b += 1;
    }
    soc
}

fn part_2(input: &str) -> usize {
    let mut pointer_a = 0;
    let mut pointer_b = 14;
    let mut soc = 0;
    for c in input.chars().enumerate() {
        let substring = &input[pointer_a..=pointer_b];
        match unique(substring) {
            None => {
                println!("is unique at {}", pointer_b);
                soc = pointer_b;
                break;
            },
            Some((i, j, c)) => println!(
                "is not unique \"{}\" at indices {} and {}",
                c, i, j
            ),
        }
        pointer_a += 1;
        pointer_b += 1;
    }
    soc
}