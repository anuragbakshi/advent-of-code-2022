enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::HashSet;

use itertools::Itertools;
use Dir::*;

impl Dir {
    fn dx(&self) -> i32 {
        match self {
            Up | Down => 0,
            Left => -1,
            Right => 1,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Left | Right => 0,
            Down => -1,
            Up => 1,
        }
    }
}

struct Move(Dir, u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn follow_one(&mut self, other: &Pos) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            self.y -= dy.signum();
            self.x -= dx.signum();
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let (d, n) = l.split_whitespace().collect_tuple().unwrap();

            let n = n.parse().unwrap();
            let d = match d {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!(),
            };

            Move(d, n)
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = parse_input(input);
    let mut visited = HashSet::new();

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    visited.insert(tail);

    for Move(d, n) in moves {
        let dx = d.dx();
        let dy = d.dy();

        for i in 0..n {
            head.x += dx;
            head.y += dy;

            tail.follow_one(&head);
            visited.insert(tail);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let moves = parse_input(input);
    let mut visited = HashSet::new();

    let mut rope = [Pos { x: 0, y: 0 }; 10];
    visited.insert(rope[9]);

    for Move(d, n) in moves {
        let dx = d.dx();
        let dy = d.dy();

        for i in 0..n {
            rope[0].x += dx;
            rope[0].y += dy;

            for x in 1..10 {
                rope[x].follow_one(&rope[x - 1].clone());
            }

            visited.insert(rope[9]);
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
