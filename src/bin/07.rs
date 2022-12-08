#![feature(map_try_insert)]

#[macro_use]
extern crate scan_fmt;

use std::collections::*;
use std::*;

use itertools::Itertools;

#[derive(Debug)]
struct Dir<'a> {
    subdirs: HashMap<&'a str, Dir<'a>>,
    files: HashMap<&'a str, u32>,
}

impl Dir<'_> {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

fn parse_input(input: &str) -> Dir {
    let mut lines = input.lines();

    let mut root = Dir::new();
    let mut current_path = vec![&mut root as *mut Dir];

    loop {
        match lines.next() {
            None => break,
            Some(line) => match line {
                "$ ls" => (),
                "$ cd /" => current_path.truncate(1),
                "$ cd .." => drop(current_path.pop()),
                line => unsafe {
                    let &top = current_path.last().unwrap();
                    match line.strip_prefix("$ cd ") {
                        Some(dir_name) => {
                            let next_dir = (*top).subdirs.get_mut(dir_name).unwrap();
                            current_path.push(next_dir);
                        }
                        None => match line.strip_prefix("dir ") {
                            Some(dir_name) => drop((*top).subdirs.try_insert(dir_name, Dir::new())),
                            None => {
                                let (size, name) = line.split_whitespace().collect_tuple().unwrap();
                                let size = size.parse::<u32>().unwrap();

                                (*top).files.insert(name, size);
                            }
                        },
                    }
                },
            },
        }
    }

    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = parse_input(input);

    fn size(dir: &Dir) -> (u32, u32) {
        let file_size: u32 = dir.files.iter().map(|(_, s)| s).sum();
        let (subdir_size, subdir_total) = dir.subdirs.iter().fold((0, 0), |acc, (_, d)| {
            let (d_size, d_total) = size(d);
            (acc.0 + d_size, acc.1 + d_total)
        });

        let dir_size = file_size + subdir_size;
        if dir_size <= 100000 {
            (dir_size, subdir_total + dir_size)
        } else {
            (dir_size, subdir_total)
        }
    }

    let (_, total) = size(&root);

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = parse_input(input);

    fn size(dir: &Dir) -> u32 {
        let file_size: u32 = dir.files.iter().map(|(_, s)| s).sum();
        let subdir_size: u32 = dir.subdirs.iter().map(|(_, s)| size(s)).sum();
        file_size + subdir_size
    }

    let used_size = size(&root);
    let free_size = 70000000 - used_size;
    let needed_size = 30000000 - free_size;

    fn find_min(needed_size: u32, dir: &Dir) -> (u32, u32) {
        let file_size: u32 = dir.files.iter().map(|(_, s)| s).sum();
        let (subdir_size, subdir_best) = dir.subdirs.iter().fold((0, 70000000), |acc, (_, d)| {
            let (d_size, d_best) = find_min(needed_size, d);
            (acc.0 + d_size, cmp::min(acc.1, d_best))
        });

        let dir_size = file_size + subdir_size;
        if dir_size >= needed_size {
            (dir_size, cmp::min(subdir_best, dir_size))
        } else {
            (dir_size, subdir_best)
        }
    }

    let (_, best) = find_min(needed_size, &root);

    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
