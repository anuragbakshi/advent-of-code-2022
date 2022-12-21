use itertools::Itertools;
use ndarray::*;
use std::*;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Path = Vec<Point>;

type Grid = Array2<u8>;

fn print_grid(grid: &Grid) {
    for x in 0..grid.len_of(Axis(0)) {
        for y in 0..grid.len_of(Axis(1)) {
            let c = match grid[[x, y]] {
                0 => '.',
                1 => '#',
                2 => 'o',
                _ => panic!(),
            };

            print!("{c}");
        }

        println!();
    }
}

const EMPTY: u8 = 0;
const WALL: u8 = 1;
const SAND: u8 = 2;

fn parse_input(input: &str) -> Vec<Path> {
    let mut paths = vec![];
    for l in input.lines() {
        let mut path = vec![];
        for p in l.split(" -> ") {
            let (x, y) = p.split(",").collect_tuple().unwrap();
            path.push(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }

        paths.push(path);
    }

    paths
}

fn build_grid(paths: &Vec<Path>, with_floor: bool) -> Grid {
    let max_x = paths.iter().flatten().map(|p| p.x).max().unwrap();
    let max_y = paths.iter().flatten().map(|p| p.y).max().unwrap();

    let mut grid = Array2::zeros((1000, 1000));

    for path in paths {
        let mut p_it = path.iter();
        let mut last = p_it.next().unwrap();

        while let Some(p) = p_it.next() {
            let start_x = cmp::min(last.x, p.x);
            let start_y = cmp::min(last.y, p.y);
            let end_x = cmp::max(last.x, p.x);
            let end_y = cmp::max(last.y, p.y);

            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    grid[[x, y]] = WALL;
                }
            }

            last = p;
        }
    }

    if with_floor {
        for x in 0..grid.len_of(Axis(0)) {
            grid[[x, max_y + 2]] = WALL;
        }
    }

    grid
}

fn put_one_sand(grid: &mut Grid) -> bool {
    let mut pos = Point { x: 500, y: 0 };
    loop {
        if pos.x == 0 || pos.x == grid.len_of(Axis(0)) - 1 || pos.y == grid.len_of(Axis(1)) - 1 {
            return false;
        }

        if grid[[pos.x, pos.y + 1]] == EMPTY {
            pos.y += 1;
            continue;
        } else if grid[[pos.x - 1, pos.y + 1]] == EMPTY {
            pos.x -= 1;
            pos.y += 1;
            continue;
        } else if grid[[pos.x + 1, pos.y + 1]] == EMPTY {
            pos.x += 1;
            pos.y += 1;
            continue;
        } else {
            break;
        }
    }

    grid[[pos.x, pos.y]] = SAND;
    true
}

fn num_sand_collected(grid: &mut Grid) -> u32 {
    while grid[[500, 0]] == EMPTY && put_one_sand(grid) {}

    let mut num_sand = 0;
    for &c in grid.iter() {
        if c == SAND {
            num_sand += 1
        }
    }

    num_sand
}

pub fn part_one(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let mut grid = build_grid(&paths, false);

    Some(num_sand_collected(&mut grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let paths = parse_input(input);
    let mut grid = build_grid(&paths, true);

    Some(num_sand_collected(&mut grid))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
