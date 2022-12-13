use std::fs;
use grid::*;
// https://crates.io/crates/pathfinding
use pathfinding::prelude::dijkstra;

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&inputs));
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(&self, map: Grid<usize>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut height = 99;
        if x < map.rows() && y < map.cols() {
            height = *map.get(x, y).unwrap();
        }
        let mut neighbours: Vec<(Pos, usize)> = Vec::new(); // create neighbours vector

        // check up
        if x > 0 {
            let x_up = x - 1;
            if x_up < map.rows() && y < map.cols() {
                let up_height = *map.get(x_up, y).unwrap();
                if up_height <= height + 1 {
                    neighbours.push((Pos(x_up, y), 1));
                }
            }
        }
        // check down
        let x_down = x + 1;
        if x_down < map.rows() && y < map.cols() {
            let down_height = *map.get(x_down, y).unwrap();
            if down_height <= height + 1 {
                neighbours.push((Pos(x_down, y), 1));
            }
        }

        //check right
        let y_right = y + 1;
        if x < map.rows() && y_right < map.cols() {
            let right_height = *map.get(x, y_right).unwrap();
            if right_height <= height + 1 {
                neighbours.push((Pos(x, y_right), 1));
            }
        }

        //check left
        if y > 0 {
            let y_left = y - 1;
            if x < map.rows() && y_left < map.cols() {
                let left_height = *map.get(x, y_left).unwrap();
                if left_height <= height + 1 {
                    neighbours.push((Pos(x, y_left), 1));
                }
            }
        }
        return neighbours;
    }
}

fn make_map(input: &str) -> (Grid<usize>, Pos, Pos) {
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    let mut grid_vec: Vec<usize> = Vec::new();
    let (mut row_length, mut row, mut col) = (usize::MAX, 0, 0);
    for char in input.chars().map(|c| c as u8) {
        match char {
            b'a'..=b'z' => grid_vec.push((char - b'a') as usize),
            b'S' => {
                grid_vec.push(0);
                start = Pos(row, col);
            }
            b'E' => {
                grid_vec.push((b'z' - b'a') as usize);
                end = Pos(row, col);
            }
            _ => {
                if col > row_length || row_length == usize::MAX {
                    row_length = col
                }
            }
        };
        if row_length == col {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    let grid: Grid<usize> = Grid::from_vec(grid_vec, row_length);
    return (grid, start, end);
}

fn part1(input: &str) -> i16 {
    let mut result = 0;
    
    let (map, start, end) = make_map(input);
    let path = dijkstra(&start, |p| p.neighbours(map.clone()), |p| *p == end);
    let mut count = 0;
    let (steps, _) = path.unwrap();
    for _step in steps {
        count += 1;
    }
    result = count - 1;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 31);
    }
}