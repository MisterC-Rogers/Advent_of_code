use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}


fn part1(input: &str) -> String {
    let trees = parse_trees(input);
    let mut count = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let height = trees[row][col];
            if trees[..row].iter().all(|x| x[col] < height)
                || trees[row][..col].iter().all(|x| x < &height)
                || trees[row + 1..].iter().all(|x| x[col] < height)
                || trees[row][col + 1..].iter().all(|x| x < &height)
            {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let trees = parse_trees(input);
    let mut count = 0;

    for row in 0..trees.len() {
        for col in 0..trees[row].len() {
            let mut curr_tree = (1, trees[row][col]);
            row_view_score(&mut curr_tree, trees[..row].iter().map(|x| x[col]).rev());
            row_view_score(&mut curr_tree, trees[row][..col].iter().rev().copied());
            row_view_score(&mut curr_tree, trees[row + 1..].iter().map(|x| x[col]));
            row_view_score(&mut curr_tree, trees[row][col + 1..].iter().copied());
            count = count.max(curr_tree.0);
        }
    }

    count.to_string()
}

fn parse_trees(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|i| {
            i.chars()
                .filter(|x| x.is_ascii_digit())
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as usize)
                .collect()
        })
        .collect()
}

fn row_view_score((view_score, height): &mut (usize, usize), row_of_trees: impl Iterator<Item=usize>) {
    let mut score = 0;

    for i in row_of_trees {
        score += 1;
        if i >= *height {
            break;
        }
    }

    *view_score *= score;
}