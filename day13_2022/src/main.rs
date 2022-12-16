use std::fs;
use serde_json::{json, Value};
use std::cmp::Ordering;

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}


fn extract_data_in_pairs(input: &str) -> Vec<(Value, Value)> {
    let mut packet_pairs: Vec<(Value, Value)> = Vec::new();
    for packet in input.split("\n\n") {
        let (left, right) = packet.split_once("\n").unwrap();
        let left_data: Value = serde_json::from_str(left).unwrap();
        let right_data: Value = serde_json::from_str(right).unwrap();
        packet_pairs.push((left_data, right_data));
    }
    return packet_pairs;
}

fn compare_pairs(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            if left.as_u64() == right.as_u64() {
                None
            } else if left.as_u64() < right.as_u64() {
                Some(true)
            } else {
                Some(false)
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            if left.is_empty() || right.is_empty() {
                if left.len() == right.len() {
                    None
                } else if left.len() < right.len() {
                    Some(true)
                } else {
                    Some(false)
                }
            } else if let Some(value) = compare_pairs(&left[0], &right[0]) {
                Some(value)
            } else {
                compare_pairs(&json!(left[1..]), &json!(right[1..]))
            }
        }
        (Value::Number(left), Value::Array(right)) => {
            compare_pairs(&json!(vec![left]), &json!(right))
        }
        (Value::Array(left), Value::Number(right)) => {
            compare_pairs(&json!(left), &json!(vec![right]))
        }
        _ => Some(true),
    }
}

fn part1(input: &str) -> String {
    let mut result = 0;
    let packets = extract_data_in_pairs(input);
    let mut pair_index = 1;
    for pair in packets {
        if compare_pairs(&pair.0, &pair.1) == Some(true) {
            result += pair_index;
        }
        pair_index += 1;
    }
    result.to_string()
}


fn extract_data(input: &str) -> Vec<Value> {
    let mut packets: Vec<Value> = Vec::new();
    for packet in input.split('\n') {
        if packet.is_empty() {
            continue;
        }
        let data: Value = serde_json::from_str(packet).unwrap();
        packets.push(data);
    }
    packets
}

fn add_divider_packets(input: Vec<Value>) -> Vec<Value> {
    let mut data = input;
    data.push(json!([[2]]));
    data.push(json!([[6]]));
    data
}

fn part2(input: &str) -> String {
    let mut result = 1;
    let mut packets = extract_data(input);
    packets = add_divider_packets(packets);

    packets.sort_by(|a, b| match compare_pairs(a, b) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        _ => Ordering::Equal,
    });
    let mut data_index = 1;
    for data in &packets {
        if data.get(0).is_some()
            && data.get(1).is_none()
            && data.get(0).unwrap().get(0).is_some()
            && data.get(0).unwrap().get(1).is_none()
            && data.get(0).unwrap().get(0).unwrap().is_number()
            && (data.get(0).unwrap().get(0).unwrap() == 2
                || data.get(0).unwrap().get(0).unwrap() == 6)
        {
            result *= data_index;
        }

        data_index += 1;
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), "13");
    }
    
    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), "140");
    }
}