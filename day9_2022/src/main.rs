use std::fs;
// needed because custom Structs lose builtin traits
use derive_more::{Add, AddAssign, Mul, Sub};
use std::{collections::HashSet, vec};

fn main() {
    let file = fs::read_to_string("./sample.txt").unwrap();
    let file_2 = fs::read_to_string("./sample2.txt").unwrap();
    // let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("part one: {}", process(&file, 1));
    println!("part two: {}", process(&file_2, 9));
    // println!("part one: {}, part two: {}", process(&inputs, 1), process(&inputs, 9));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    /*
        signum will return the sign of the number
        0 if the number is zero 
        1 if the number is positive 
        -1 if the number is negative
    */
    fn normalize(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn max(&self) -> i32 {
        self.x.max(self.y)
    }

    // Direction, count
    fn from_line(travel: &str) -> (Self, u32) {
        let (direction, count) = travel.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();

        let out = match direction {
            "R" => Self::new(1, 0),
            "L" => Self::new(-1, 0),
            "U" => Self::new(0, -1),
            "D" => Self::new(0, 1),
            _ => panic!("Invalid direction"),
        };

        (out, count as u32)
    }
}

fn process(input: &str, allowed_spaces: usize) -> String {
    let head_moves = input.lines().map(Coordinate::from_line).collect::<Vec<_>>();
    // store tail position like a history
    let mut tail_positions = HashSet::new();
    // store head position moves
    let mut knots = vec![Coordinate::new(0, 0); allowed_spaces + 1];
    // starting position
    tail_positions.insert(*knots.last().unwrap());
    
    for (direction, move_count) in head_moves {
        for _ in 0..move_count {
            knots[0] += direction;
            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.abs().max() <= 1 {
                    continue;
                }
                knots[i] += diff.normalize();
            }
            tail_positions.insert(*knots.last().unwrap());
        }
    }
    tail_positions.len().to_string()
}
