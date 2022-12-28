#![feature(variant_count)]
#![feature(generators)]
#![feature(iter_from_generator)]

use std::{
    cmp::{max, min},
    collections::HashMap,
};

use enum_utils::IterVariants;
use itertools::Itertools;
use maplit::hashmap;

#[derive(Clone, IterVariants)]
enum Dir {
    North,
    South,
    West,
    East,
}

use Dir::*;

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            South => (1, 0),
            West => (0, -1),
            East => (0, 1),
        }
    }

    fn orthogonal(&self) -> [Dir; 2] {
        match self {
            North | South => [East, West],
            West | East => [North, South],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn adj(&self, dir: &Dir) -> Self {
        let (dx, dy) = dir.delta();

        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Pos> + '_ {
        std::iter::from_generator(|| {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if !(dx == 0 && dy == 0) {
                        yield Pos {
                            x: self.x + dx,
                            y: self.y + dy,
                        }
                    }
                }
            }
        })
    }
}

struct Elf {
    pos: Pos,
}

struct State {
    elfs: HashMap<Pos, Elf>,
    dirs: Vec<Dir>,
}
impl State {
    fn step(&mut self) -> bool {
        let mut proposals_from = hashmap! {};

        // step 1
        for (_, e) in self.elfs.iter() {
            let any_neighbors = e.pos.neighbors().any(|p| self.elfs.contains_key(&p));

            if any_neighbors {
                for dir_to_try in self.dirs.iter() {
                    let can_move = !self.elfs.contains_key(&e.pos.adj(dir_to_try))
                        && dir_to_try
                            .orthogonal()
                            .iter()
                            .all(|d| !self.elfs.contains_key(&e.pos.adj(dir_to_try).adj(d)));

                    if can_move {
                        proposals_from
                            .entry(e.pos.adj(dir_to_try))
                            .or_insert(vec![])
                            .push(e.pos.clone());

                        break;
                    }
                }
            }
        }

        let mut any_moved = false;

        // step 2
        for (pos, from) in proposals_from.iter() {
            if from.len() == 1 {
                let from = &from[0];
                let mut elf = self.elfs.remove(from).unwrap();

                elf.pos = pos.clone();
                self.elfs.insert(pos.clone(), elf);

                any_moved = true;
            }
        }

        let first_dir = self.dirs.remove(0);
        self.dirs.push(first_dir);

        any_moved
    }

    fn elf_bbox(&self) -> (Pos, Pos) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for (p, _) in self.elfs.iter() {
            min_x = min(min_x, p.x);
            min_y = min(min_y, p.y);
            max_x = max(max_x, p.x);
            max_y = max(max_y, p.y);
        }

        (Pos { x: min_x, y: min_y }, Pos { x: max_x, y: max_y })
    }

    fn captured_area(&self) -> isize {
        let (min, max) = self.elf_bbox();

        (max.x - min.x + 1) * (max.y - min.y + 1)
    }

    fn print(&self) {
        let (min, max) = self.elf_bbox();

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                if self.elfs.contains_key(&Pos { x, y }) {
                    print!("#");
                } else {
                    print!(".")
                }
            }

            println!();
        }
    }
}

fn parse_input(input: &str) -> State {
    let mut elfs = hashmap! {};

    let as_vec = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(y, c)| match c {
                    '#' => Some(Elf {
                        pos: Pos {
                            x: x as isize,
                            y: y as isize,
                        },
                    }),
                    _ => None,
                })
                .collect_vec()
        })
        .concat();

    for e in as_vec {
        elfs.insert(e.pos.clone(), e);
    }

    State {
        elfs,
        dirs: Dir::iter().collect(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut state = parse_input(input);
    for _ in 0..10 {
        state.step();
    }

    Some((state.captured_area() - state.elfs.len() as isize) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = parse_input(input);

    let mut steps = 1;
    while state.step() {
        steps += 1;
    }

    Some(steps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
