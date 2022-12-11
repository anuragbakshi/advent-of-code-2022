use std::collections::*;

pub fn part_one(input: &str) -> Option<u32> {
    let l = input.len();
    for i in 4..=l {
        let c: HashSet<char> = input[i - 4..i].chars().collect();
        if c.len() == 4 {
            return Some(i as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let l = input.len();
    for i in 14..=l {
        let c: HashSet<char> = input[i - 14..i].chars().collect();
        if c.len() == 14 {
            return Some(i as u32);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
