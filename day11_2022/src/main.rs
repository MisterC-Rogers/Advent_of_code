use std::fs;
use std::{cell::RefCell, collections::VecDeque};


fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    inspected: RefCell<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
enum Value {
    Old,
    Number(u64),
}

impl Value {
    fn parse(inp: &str) -> Self {
        if inp == "old" {
            return Self::Old;
        }
        Self::Number(inp.parse::<u64>().unwrap())
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value),
    Multiply(Value),
}

impl Operation {
    fn parse(inp: &str) -> Self {
        let mut parts = inp.split_whitespace();
        debug_assert_eq!(parts.next().unwrap(), "old");

        match parts.next().unwrap() {
            "*" => Self::Multiply(Value::parse(parts.next().unwrap())),
            "+" => Self::Add(Value::parse(parts.next().unwrap())),
            _ => panic!("Unsuppored operation"),
        }
    }

    fn process(&self, old: u64) -> u64 {
        match self {
            Self::Add(Value::Old) => old + old,
            Self::Add(Value::Number(n)) => old + n,
            Self::Multiply(Value::Old) => old * old,
            Self::Multiply(Value::Number(n)) => old * n,
        }
    }
}

#[derive(Debug)]
struct Test {
    divid_by: u64,
    monkey: [usize; 2],
}

impl Test {
    fn parse(inp: &[&str]) -> Self {
        let divid_by = inp[0].split_once("by ").unwrap().1.parse::<u64>().unwrap();

        let mut monkey = [0; 2];
        for (i, line) in inp[1..].iter().enumerate() {
            let monkey_id = line
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            monkey[i] = monkey_id;
        }

        Self { divid_by, monkey }
    }

    fn process(&self, item: u64) -> usize {
        if item % self.divid_by == 0 {
            return self.monkey[0];
        }
        self.monkey[1]
    }
}

fn parse_monkeys(raw: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for i in raw.lines().collect::<Vec<_>>().chunks(7) {
        let items = i[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<_>>();

        let operation = Operation::parse(i[2].split_once(" = ").unwrap().1);
        let test = Test::parse(&i[3..6]);

        monkeys.push(Monkey {
            items: RefCell::new(items),
            inspected: RefCell::new(0),
            operation,
            test,
        });
    }

    monkeys
}

fn part1(input: &str) -> String {
    let mut monkeys = parse_monkeys(&input);
    let rounds = 20;
    let worry_manager = 3;
    
    for _ in 0..rounds {
        for monkey in &monkeys {
            while let Some(mut item) = monkey.items.borrow_mut().pop_front() {
                *monkey.inspected.borrow_mut() += 1;
                item = monkey.operation.process(item) / worry_manager;
                monkeys[monkey.test.process(item)]
                    .items
                    .borrow_mut()
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|x| -(*x.inspected.borrow() as isize));
    let a = *monkeys[0].inspected.borrow();
    let b = *monkeys[1].inspected.borrow();

    (a * b).to_string()
}

fn part2(input: &str) -> String {
    let mut monkeys = parse_monkeys(&input);
    let rounds = 10_000;
    let worry_manager = monkeys.iter().map(|x| x.test.divid_by).product::<u64>();
    
    for _ in 0..rounds {
        for monkey in &monkeys {
            while let Some(mut item) = monkey.items.borrow_mut().pop_front() {
                *monkey.inspected.borrow_mut() += 1;
                item = monkey.operation.process(item) % worry_manager;
                monkeys[monkey.test.process(item)]
                    .items
                    .borrow_mut()
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|x| -(*x.inspected.borrow() as isize));
    let a = *monkeys[0].inspected.borrow();
    let b = *monkeys[1].inspected.borrow();

    (a * b).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "10605");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), "2713310158");
    }
}