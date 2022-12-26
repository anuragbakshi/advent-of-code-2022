#[macro_use]
extern crate scan_fmt;

#[macro_use]
extern crate maplit;

use std::{collections::HashMap, str::Lines};

use itertools::Itertools;

#[derive(Debug, enum_utils::FromStr, PartialEq, Eq, Hash)]
#[enumeration(case_insensitive)]
enum Item {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Recipe = HashMap<Item, u32>;

type Blueprint = HashMap<Item, Recipe>;

fn parse_blueprint(lines: &mut Lines) -> Option<Blueprint> {
    let _ = scan_fmt!(lines.next()?, "Blueprint {}:", u32).unwrap();
    let mut blueprint: Blueprint = hashmap! {};

    for _ in 0..4 {
        let line = lines
            .next()
            .unwrap()
            .strip_suffix(".")
            .unwrap()
            .split_whitespace()
            .collect_vec();

        if line.len() == 6 {
            blueprint.insert(
                line[1].parse().unwrap(),
                hashmap! { line[5].parse().unwrap() => line[4].parse().unwrap() },
            );
        } else {
            blueprint.insert(
                line[1].parse().unwrap(),
                hashmap! {
                    line[5].parse().unwrap() => line[4].parse().unwrap(),
                    line[8].parse().unwrap() => line[7].parse().unwrap(),
                },
            );
        }
    }

    Some(blueprint)
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut lines = input.lines();
    let mut blueprints = vec![];
    while let Some(b) = parse_blueprint(&mut lines) {
        blueprints.push(b);
        let _ = lines.next();
    }

    blueprints
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_input(input);
    println!("{:?}", blueprints);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
