use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("part one => {}", part_1(&file));
    println!("part one => {}", part_2(&file));
}

fn part_1(input: &str) -> String {
    let mut recon = 0;

    for (a, b) in assignment_loop(input) {
        recon += ((a.contains(b.start()) && a.contains(b.end()))
            || (b.contains(a.start()) && b.contains(a.end()))) as usize;
    }

    recon.to_string()
}

fn part_2(input: &str) -> String {
    let mut recon = 0;

    for (a, b) in assignment_loop(input) {
        recon += (a.start().max(b.start()) <= a.end().min(b.end())) as usize;
    }

    recon.to_string()
}


fn assignment_loop(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input.trim()
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (split_range(a), split_range(b)))
        .collect()
}

fn split_range(range: &str) -> RangeInclusive<u32> {
    let mut range = range.split('-').map(|x| x.parse::<u32>().unwrap());
    range.next().unwrap()..=range.next().unwrap()
}