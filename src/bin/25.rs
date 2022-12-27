use std::str::FromStr;

use itertools::Itertools;

struct Snafu {
    digits: Vec<i8>,
}

impl Snafu {
    fn to_int(&self) -> i64 {
        self.digits.iter().fold(0, |acc, d| acc * 5 + (*d as i64))
    }

    fn max_repr(num_digits: usize) -> i64 {
        let mut max_repr = 0;
        for _ in 0..num_digits {
            max_repr = max_repr * 5 + 2;
        }

        max_repr
    }

    fn min_repr(num_digits: usize) -> i64 {
        let mut max_repr = 0;
        for _ in 0..num_digits {
            max_repr = max_repr * 5 - 2;
        }

        max_repr
    }

    fn of_int(int: i64) -> Self {
        let mut num_digits = 1;
        while Self::max_repr(num_digits) < int {
            num_digits += 1;
        }

        let mut s = Snafu {
            digits: vec![0i8; num_digits],
        };

        for i in 0..num_digits {
            for d in -2..=2 {
                s.digits[i] = d;

                let diff = int - s.to_int();
                let min = Self::min_repr(num_digits - i - 1);
                let max = Self::max_repr(num_digits - i - 1);
                if min <= diff && diff <= max {
                    break;
                }
            }
        }

        s
    }
}

impl FromStr for Snafu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(|c| match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("bad digit {c}"),
            })
            .collect_vec();

        Ok(Snafu { digits })
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        self.digits
            .iter()
            .map(|d| match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("bad digit {d}"),
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let input = input
        .lines()
        .map(|s| s.parse::<Snafu>().unwrap())
        .collect_vec();

    let total = input.iter().map(|s| s.to_int()).sum::<i64>();

    Some(Snafu::of_int(total).to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
