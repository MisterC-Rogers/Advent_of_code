use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs.txt").unwrap();
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

fn part1(file: &str) -> i32 {
    let lines = file
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    let mut dir_size_100k = 0;

    get_dir_size_part1(&lines, &mut 0, &mut dir_size_100k);
    dir_size_100k
}

fn part2(file: &str) -> i32 {
    let lines = file
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    let mut dir_sizes = Vec::<i32>::new();

    let total_dir_size = get_dir_size_part2(&lines, &mut 0, &mut dir_sizes);
    let mut min_dir_size = Option::None::<i32>;

    for dir_size in dir_sizes {
        if (70_000_000 - total_dir_size + dir_size) >= 30_000_000
            && (min_dir_size.is_none() || dir_size < min_dir_size.unwrap())
        {
            min_dir_size = Some(dir_size);
        }
    }

    min_dir_size.unwrap()
}

fn get_dir_size_part1(lines: &[String], index: &mut usize, dir_size_100k: &mut i32) -> i32 {
    let mut dir_size = 0;

    while *index < lines.len() {
        let line = lines.get(*index).unwrap();
        *index += 1;

        if line.contains("$ cd") {
            let dir = *line.split(' ').collect::<Vec<&str>>().get(2).unwrap();

            if dir != ".." {
                let child_dir_size = get_dir_size_part1(lines, index, dir_size_100k);
                dir_size += child_dir_size;

                if child_dir_size <= 100_000 {
                    *dir_size_100k += child_dir_size;
                }
            } else {
                return dir_size;
            }
        } else if line == "$ ls" || line.contains("dir ") {
            // Ignore
        } else {
            let file_size = line.split(' ').next().unwrap().parse::<i32>().unwrap();
            dir_size += file_size;
        }
    }

    dir_size
}

fn get_dir_size_part2(lines: &[String], index: &mut usize, dir_sizes: &mut Vec<i32>) -> i32 {
    let mut dir_size = 0;

    while *index < lines.len() {
        let line = lines.get(*index).unwrap();
        *index += 1;

        if line.contains("$ cd") {
            let dir = *line.split(' ').collect::<Vec<&str>>().get(2).unwrap();

            if dir != ".." {
                let child_dir_size = get_dir_size_part2(lines, index, dir_sizes);
                dir_size += child_dir_size;

                dir_sizes.push(child_dir_size);
            } else {
                return dir_size;
            }
        } else if line == "$ ls" || line.contains("dir ") {
            // Ignore
        } else {
            let file_size = line.split(' ').next().unwrap().parse::<i32>().unwrap();
            dir_size += file_size;
        }
    }

    dir_size
}
