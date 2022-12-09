use std::fs;
// needed because custom Structs lose builtin traits
use derive_more::{Add, AddAssign, Mul, Sub};
use std::{collections::HashSet, vec};

fn main() {
    let file = fs::read_to_string("./sample.txt").unwrap();
    let file_2 = fs::read_to_string("./sample2.txt").unwrap();
    let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("part one: {}", part1(&file));
    println!("part two: {}", part2(&file_2));
    println!("part one: {}, part two: {}", part1(&inputs), part2(&inputs));
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

fn part1(input: &str) -> String {
    let head_moves = input.lines().map(Coordinate::from_line).collect::<Vec<_>>();
    // store tail position like a history
    let mut tail_positions = HashSet::new();
    // store head position can only hold at max two positions because the tail has to touch
    let mut knots = vec![Coordinate::new(0, 0); 2];
    // starting position
    tail_positions.insert(*knots.last().unwrap());
    
    for (dir, move_count) in head_moves {
        for _ in 0..move_count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                // get the abs and get the sign of the num
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

fn part2(input: &str) -> String {
    let head_moves = input.lines().map(Coordinate::from_line).collect::<Vec<_>>();
    let mut tail_positions = HashSet::new();
    let mut knots = vec![Coordinate::new(0, 0); 10];

    tail_positions.insert(*knots.last().unwrap());
    for (dir, move_count) in head_moves {
        for _ in 0..move_count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                // get the abs and get the sign of the num
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