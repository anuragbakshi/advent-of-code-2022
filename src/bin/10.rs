use itertools::Itertools;

enum Instr {
    Noop,
    Addx(i32),
}

use Instr::*;

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|l| match l {
            "noop" => Noop,
            l => {
                let (_, n) = l.split_whitespace().collect_tuple().unwrap();
                Addx(n.parse().unwrap())
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i32> {
    let instrs = parse_input(input);

    let mut cycle = 1;
    let mut reg = vec![1, 1];

    for instr in instrs {
        match instr {
            Noop => {
                cycle += 1;
                reg.push(*reg.last().unwrap())
            }
            Addx(n) => {
                cycle += 1;
                reg.push(*reg.last().unwrap());
                cycle += 1;
                reg.push(*reg.last().unwrap() + n)
            }
        }
    }

    let interesting = [20, 60, 100, 140, 180, 220];
    let ans = interesting.into_iter().map(|i| (i as i32) * reg[i]).sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<i32> {
    let instrs = parse_input(input);

    let mut cycle = 1;
    let mut reg = vec![1, 1];

    for instr in instrs {
        match instr {
            Noop => {
                cycle += 1;
                reg.push(*reg.last().unwrap())
            }
            Addx(n) => {
                cycle += 1;
                reg.push(*reg.last().unwrap());
                cycle += 1;
                reg.push(*reg.last().unwrap() + n)
            }
        }
    }

    for i in 1..=240 {
        let pix = ((i as i32) - 1) % 40;
        if pix.abs_diff(reg[i] % 40) <= 1 {
            print!("#");
        } else {
            print!(" ");
        }

        if i % 40 == 0 {
            println!();
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
