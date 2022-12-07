#[macro_use]
extern crate scan_fmt;

use std::*;

pub fn part_one(input: &str) -> Option<u32> {
    let mut subsets = 0;

    for l in input.lines() {
        let (start1, end1, start2, end2) =
            scan_fmt!(l, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();

        if (start1 <= start2 && end2 <= end1) || (start2 <= start1 && end1 <= end2) {
            subsets += 1;
        }
    }

    Some(subsets)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlaps = 0;

    for l in input.lines() {
        let (start1, end1, start2, end2) =
            scan_fmt!(l, "{d}-{d},{d}-{d}", u32, u32, u32, u32).unwrap();

        if (start1 <= start2 && start2 <= end1) || (start2 <= start1 && start1 <= end2) {
            overlaps += 1;
        }
    }

    Some(overlaps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
