use std::fs;
use std::collections::BTreeMap;
use itertools::Itertools; 

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}

fn from_line(instruction: &str) -> (u32, i32) {
    if instruction.contains("noop"){
        (1, 0)
    } else {
        let (call, value) = instruction.split_once(' ').unwrap();
        let value: i32 = value.parse().unwrap();

        let cycle = match call {
            "noop" => 1,
            "addx" => 2,
            _ => panic!("Invalid instruction"),
        };
        (cycle, value as i32)
    }
}

fn part1(input: &str) -> String {
    let notable_cycles = [20, 60, 100, 140, 180, 220];
    let mut scores: BTreeMap<u32, i32> = BTreeMap::new();
    let instruction_list = input.lines().map(from_line).collect::<Vec<_>>();
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;
    
    for instruction in instruction_list.iter() {
        if notable_cycles.contains(&(cycles + 1)) {
            scores.insert(
                cycles + 1,
                (cycles as i32 + 1) * x,
            );
        }

        if notable_cycles.contains(&(cycles + 2)) {
            scores.insert(
                cycles + 2,
                (cycles as i32 + 2) * x,
            );
        }

        cycles += instruction.0;
        x += instruction.1;
    }

    scores
        .iter()
        .map(|(_key, value)| value)
        .sum::<i32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let instruction_list = input.lines().map(from_line).collect::<Vec<_>>();
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;
    let mut crt_pixels: String = "".to_string();

    for instruction in instruction_list.iter() {
        for cycle_add in 0..instruction.0 {
            let pixel_id =
                (cycles as i32 + cycle_add as i32) % 40;

            if ((x - 1)..=(x + 1)).contains(&pixel_id) {
                crt_pixels.push_str("#");
            } else {
                crt_pixels.push_str(".");
            }
        }

        cycles += instruction.0;
        x += instruction.1;
    }

    crt_pixels
        .chars()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "13140");
    }
    
    #[test]
    fn part2_works() {
        assert_eq!(
            part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}

