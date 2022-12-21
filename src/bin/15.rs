#[macro_use]
extern crate scan_fmt;

struct Pos {
    x: i32,
    y: i32,
}

struct Reading {
    sensor: Pos,
    beacon: Pos,
}

impl Reading {
    fn area_covered(&self) -> i32 {
        let dx = self.sensor.x.abs_diff(self.beacon.x);
        let dy = self.sensor.y.abs_diff(self.beacon.y);
    }
}

fn parse_input(input: &str) {
    input.lines().map(|l| {
        let (sx, sy, bx, by) = scan_fmt!(
            l,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
