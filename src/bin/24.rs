use itertools::Itertools;
use maplit::hashset;
use ndarray::*;
use std::{collections::HashSet, *};

const EMPTY: u8 = 0;
const WALL: u8 = 1;
const BLIZZARD_U: u8 = 2;
const BLIZZARD_R: u8 = 4;
const BLIZZARD_D: u8 = 8;
const BLIZZARD_L: u8 = 16;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    r: i32,
    c: i32,
}

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            self.clone(),
            Pos {
                r: self.r - 1,
                c: self.c,
            },
            Pos {
                r: self.r + 1,
                c: self.c,
            },
            Pos {
                r: self.r,
                c: self.c - 1,
            },
            Pos {
                r: self.r,
                c: self.c + 1,
            },
        ]
    }
}

struct State {
    grid: Array2<u8>,
    me: HashSet<Pos>,
}

fn bit_set(val: u8, bit: u8) -> bool {
    val & bit != 0
}

impl State {
    fn print(&self) {
        for r in 0..self.grid.len_of(Axis(0)) {
            for c in 0..self.grid.len_of(Axis(1)) {
                let as_char = if self.me.contains(&Pos {
                    r: r as i32,
                    c: c as i32,
                }) {
                    'E'
                } else {
                    match self.grid[[r, c]] {
                        EMPTY => '.',
                        WALL => '#',
                        BLIZZARD_U => '^',
                        BLIZZARD_R => '>',
                        BLIZZARD_D => 'v',
                        BLIZZARD_L => '<',
                        x => x.count_ones().to_string().chars().next().unwrap(),
                    }
                };

                print!("{as_char}");
            }

            println!();
        }
    }

    fn step(&self) -> Self {
        let mut next = Self {
            grid: Array2::zeros(self.grid.raw_dim()),
            me: HashSet::new(),
        };

        let n_rows = self.grid.len_of(Axis(0));
        let n_cols = self.grid.len_of(Axis(1));

        for r in 0..n_rows {
            for c in 0..n_cols {
                let curr = self.grid[[r, c]];

                if curr == WALL {
                    next.grid[[r, c]] = WALL;
                } else {
                    if bit_set(curr, BLIZZARD_U) {
                        if r == 1 {
                            next.grid[[n_rows - 2, c]] |= BLIZZARD_U;
                        } else {
                            next.grid[[r - 1, c]] |= BLIZZARD_U;
                        }
                    }

                    if bit_set(curr, BLIZZARD_D) {
                        if r == n_rows - 2 {
                            next.grid[[1, c]] |= BLIZZARD_D;
                        } else {
                            next.grid[[r + 1, c]] |= BLIZZARD_D;
                        }
                    }

                    if bit_set(curr, BLIZZARD_L) {
                        if c == 1 {
                            next.grid[[r, n_cols - 2]] |= BLIZZARD_L;
                        } else {
                            next.grid[[r, c - 1]] |= BLIZZARD_L;
                        }
                    }

                    if bit_set(curr, BLIZZARD_R) {
                        if c == n_cols - 2 {
                            next.grid[[r, 1]] |= BLIZZARD_R;
                        } else {
                            next.grid[[r, c + 1]] |= BLIZZARD_R;
                        }
                    }
                }
            }
        }

        for pos in self.me.iter() {
            for adj in pos.neighbors() {
                if 0 <= adj.r
                    && adj.r < (n_rows as i32)
                    && 0 <= adj.c
                    && adj.c < (n_cols as i32)
                    && next.grid[[adj.r as usize, adj.c as usize]] == EMPTY
                {
                    next.me.insert(adj);
                }
            }
        }

        next
    }
}

fn parse_input(input: &str) -> State {
    let v = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => EMPTY,
                    '#' => WALL,
                    '^' => BLIZZARD_U,
                    '>' => BLIZZARD_R,
                    'v' => BLIZZARD_D,
                    '<' => BLIZZARD_L,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    State {
        grid: Array2::from_shape_vec((v.len(), v[0].len()), v.concat()).unwrap(),
        me: hashset! {Pos { r: 0, c: 1 }},
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut state = parse_input(input);
    let end = Pos {
        r: state.grid.len_of(Axis(0)) as i32 - 1,
        c: state.grid.len_of(Axis(1)) as i32 - 2,
    };

    let mut minutes = 0;
    loop {
        if state.me.contains(&end) {
            return Some(minutes);
        }

        minutes += 1;
        state = state.step();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = parse_input(input);

    let start = Pos { r: 0, c: 1 };
    let end = Pos {
        r: state.grid.len_of(Axis(0)) as i32 - 1,
        c: state.grid.len_of(Axis(1)) as i32 - 2,
    };

    let mut minutes = 0;

    state.me = hashset! {start.clone()};
    loop {
        if state.me.contains(&end) {
            break;
        }

        minutes += 1;
        state = state.step();
    }

    state.me = hashset! {end.clone()};
    loop {
        if state.me.contains(&start) {
            break;
        }

        minutes += 1;
        state = state.step();
    }

    state.me = hashset! {start.clone()};
    loop {
        if state.me.contains(&end) {
            break;
        }

        minutes += 1;
        state = state.step();
    }

    Some(minutes)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
