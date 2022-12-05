use Move::*;
use Outcome::*;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome_against(&self, other: &Move) -> Outcome {
        if self == other {
            Draw
        } else {
            let wins_against = match self {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            };

            if other == &wins_against {
                Win
            } else {
                Lose
            }
        }
    }

    fn for_outcome_against(&self, outcome: Outcome) -> Move {
        if Rock.outcome_against(self) == outcome {
            Rock
        } else if Paper.outcome_against(self) == outcome {
            Paper
        } else {
            Scissors
        }
    }

    fn score_against(&self, other: &Move) -> u32 {
        self.score() + self.outcome_against(other).score()
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut total_score = 0;

    for l in lines {
        let (other, me) = l.split_once(" ").unwrap();

        let other = match other {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        };

        let me = match me {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!(),
        };

        total_score += me.score_against(&other);
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    let mut total_score = 0;

    for l in lines {
        let (other, me) = l.split_once(" ").unwrap();

        let other = match other {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        };

        let me = match me {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!(),
        };

        let me = other.for_outcome_against(me);
        total_score += me.score_against(&other);
    }

    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
