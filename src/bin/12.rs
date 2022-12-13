use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use ndarray::*;

type Pos = (usize, usize);
type Grid = Array2<u8>;

fn parse_input(input: &str) -> (Grid, Pos, Pos) {
    let v = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect_vec())
        .collect_vec();

    let mut grid = Grid::from_shape_vec((v.len(), v[0].len()), v.concat()).unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..grid.len_of(Axis(0)) {
        for j in 0..grid.len_of(Axis(1)) {
            if grid[[i, j]] == 'S' as u8 {
                grid[[i, j]] = 'a' as u8;
                start = (i, j);
            } else if grid[[i, j]] == 'E' as u8 {
                grid[[i, j]] = 'z' as u8;
                end = (i, j);
            }
        }
    }

    (grid, start, end)
}

fn neighbors(g: &Grid, (i, j): Pos) -> Vec<Pos> {
    let x = g[[i, j]];
    let mut n = vec![];

    if 0 < i && g[[i - 1, j]] <= x + 1 {
        n.push((i - 1, j));
    }

    if 0 < j && g[[i, j - 1]] <= x + 1 {
        n.push((i, j - 1));
    }

    if i < g.len_of(Axis(0)) - 1 && g[[i + 1, j]] <= x + 1 {
        n.push((i + 1, j));
    }

    if j < g.len_of(Axis(1)) - 1 && g[[i, j + 1]] <= x + 1 {
        n.push((i, j + 1));
    }

    n
}

fn bfs(grid: &Grid, start: Pos, end: Pos) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([(start, 0)]);

    while let Some((curr, dist)) = to_visit.pop_front() {
        if visited.contains(&curr) {
            continue;
        }

        visited.insert(curr);

        if curr == end {
            return Some(dist);
        }

        for n in neighbors(&grid, curr) {
            to_visit.push_back((n, dist + 1));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start, end) = parse_input(input);

    bfs(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, end) = parse_input(input);
    let mut starts = vec![];

    for i in 0..grid.len_of(Axis(0)) {
        for j in 0..grid.len_of(Axis(1)) {
            if grid[[i, j]] == 'a' as u8 {
                starts.push((i, j));
            }
        }
    }

    Some(
        starts
            .iter()
            .filter_map(|&s| bfs(&grid, s, end))
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
