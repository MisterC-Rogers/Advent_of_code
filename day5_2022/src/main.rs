use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("part one => {}", process(&file, true));
    println!("part two => {}", process(&file, false));
}

fn process(input: &str, part: bool) -> String {
    let (stack, orders) = input.split_once("\n\n").unwrap();
    let mut crates = get_init_stack(stack);

    for i in orders.trim().lines() {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        if part {
            for _ in 0..count {
                let old = crates[from].remove(0);
                crates[to].insert(0, old);
            }
            continue;
        }
        
        
        let mut restacked = Vec::new();
        for _ in 0..count {
            restacked.push(crates[from].remove(0));
        }

        for i in restacked.iter().rev() {
            crates[to].insert(0, *i);
        }
    }

    let mut output = String::new();
    for i in crates.iter().filter(|x| !x.is_empty()) {
        output.push(i[0]);
    }
    output
}

fn get_init_stack(input: &str) -> Vec<Vec<char>> {
    let mut output = vec![Vec::new(); 9];

    for i in input.lines().filter(|x| !x.starts_with(" 1")) {
        for j in 0..9 {
            if let Some(x) = i.chars().nth(1 + j * 4) {
                if x.is_whitespace() {
                    continue;
                }
                output[j].push(x);
            }
        }
    }
    output
}