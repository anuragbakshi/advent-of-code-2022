#[macro_use]
extern crate scan_fmt;

#[macro_use]
extern crate maplit;

use std::{collections::HashMap, str::Lines};

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
    let mut blueprint = hashmap! {};

    for _ in 0..4 {
        let line = lines.next().unwrap();
        println!("{line}");
        match scan_fmt!(
            line,
            "  Each {/.*/} robot costs {d} {/.*/}.",
            Item,
            u32,
            Item
        ) {
            Ok((robot, n, item)) => {
                blueprint.insert(robot, hashmap! {item => n});
            }
            Err(_) => {
                let (robot, n1, item1, n2, item2) = scan_fmt!(
                    line,
                    "  Each {} robot costs {d} {} and {d} {}.",
                    Item,
                    u32,
                    Item,
                    u32,
                    Item
                )
                .unwrap();

                blueprint.insert(robot, hashmap! {item1 => n1, item2 => n2});
            }
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
    fn test() {
        let x = "Ore".parse::<Item>();

        println!("{x:?}");

        let (a, b, c) = scan_fmt!(
            "  Each ore robot costs 4 ore.",
            "  Each {/.*/} robot costs {d} {/.*/}.",
            Item,
            u32,
            Item
        )
        .unwrap();

        println!("{a:?}, {b:?}, {c:?}");
    }

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
