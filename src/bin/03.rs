use std::collections::HashSet;

use itertools::Itertools;

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - b'a' + 1,
        'A'..='Z' => (c as u8) - b'A' + 27,
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0u32;

    for l in input.lines() {
        let n = l.len();
        let (left, right) = l.split_at(n / 2);

        let left: HashSet<char> = left.chars().collect();
        let right: HashSet<char> = right.chars().collect();

        let common = left.intersection(&right).next().unwrap();
        total += priority(*common) as u32;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut it = input.lines().peekable();
    let mut total = 0u32;

    while it.peek().is_some() {
        let (a, b, c) = it.next_tuple().unwrap();

        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let c: HashSet<char> = c.chars().collect();

        let common = a.iter().find(|x| b.contains(x) && c.contains(x)).unwrap();

        total += priority(*common) as u32;
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
