#![feature(iter_next_chunk)]

#[macro_use]
extern crate scan_fmt;

use std::*;

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parse_input(input: &str) -> State {
    let mut state = State {
        crates: vec![],
        moves: vec![],
    };

    let mut n_rows = 0;
    let mut n_stacks = 0;
    for (i, l) in input.lines().enumerate() {
        if l.starts_with(" 1 ") {
            n_rows = i;
            n_stacks = (l.len() + 1) / 4;
            break;
        }
    }

    state.crates.resize(n_stacks, vec![]);
    for l in input.lines().take(n_rows) {
        let l = l.as_bytes();
        for i in 0..n_stacks {
            let c = l[4 * i + 1] as char;
            if c.is_ascii_alphabetic() {
                state.crates[i].push(c);
            }
        }
    }

    for m in input.lines().skip(n_rows + 2) {
        let (n, from, to) = scan_fmt!(m, "move {d} from {d} to {d}", usize, usize, usize).unwrap();
        state.moves.push(Move {
            n,
            from: from - 1,
            to: to - 1,
        });
    }

    for s in &mut state.crates {
        s.reverse()
    }

    state
}

pub fn part_one(input: &str) -> Option<String> {
    let mut state = parse_input(input);

    for m in state.moves {
        for _ in 0..m.n {
            let c = state.crates[m.from].pop().unwrap();
            state.crates[m.to].push(c);
        }
    }

    let mut tops: String = "".to_string();
    for s in state.crates {
        tops.push(*s.last().unwrap());
    }

    Some(tops)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut state = parse_input(input);

    for m in state.moves {
        let l = state.crates[m.from].len();
        for i in 0..m.n {
            let c = state.crates[m.from][l - m.n + i];
            state.crates[m.to].push(c);
        }

        state.crates[m.from].truncate(l - m.n);
    }

    let mut tops: String = "".to_string();
    for s in state.crates {
        tops.push(*s.last().unwrap());
    }

    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
