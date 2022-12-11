#![feature(drain_filter)]

#[macro_use]
extern crate scan_fmt;

use std::*;

use itertools::Itertools;

#[derive(Debug)]
enum Token {
    Old,
    Const(u64),
}

use Token::*;

impl Token {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Old => old,
            Const(n) => *n,
        }
    }
}

impl str::FromStr for Token {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Old)
        } else {
            let n = s.parse::<u64>()?;
            Ok(Const(n))
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

use Operator::*;

impl str::FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Add),
            "*" => Ok(Mul),
            _ => Err(format!("bad operator {s}").to_owned()),
        }
    }
}

#[derive(Debug)]
struct Operation {
    a: Token,
    b: Token,
    op: Operator,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        let a = self.a.eval(old);
        let b = self.b.eval(old);

        match self.op {
            Add => a + b,
            Mul => a * b,
        }
    }
}

#[derive(Debug)]
struct Test {
    div_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn check(&self, n: u64) -> usize {
        if n % self.div_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    n: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    n_inspected: u64,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    fn parse_monkey(lines: &mut str::Lines) -> Option<Monkey> {
        let n = scan_fmt!(lines.next()?, "Monkey {d}:", usize).unwrap();
        let items = {
            let items = lines.next()?.strip_prefix("  Starting items: ").unwrap();
            items
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec()
        };

        let operation = {
            let (a, op, b) = scan_fmt!(
                lines.next()?,
                "  Operation: new = {} {} {}",
                String,
                String,
                String
            )
            .unwrap();

            Operation {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                op: op.parse().unwrap(),
            }
        };

        let test = {
            let div_by = scan_fmt!(lines.next()?, "  Test: divisible by {d}", u64).unwrap();

            let if_true =
                scan_fmt!(lines.next()?, "    If true: throw to monkey {d}", usize).unwrap();

            let if_false =
                scan_fmt!(lines.next()?, "    If false: throw to monkey {d}", usize).unwrap();

            let _ = lines.next();

            Test {
                div_by,
                if_true,
                if_false,
            }
        };

        Some(Monkey {
            n,
            items,
            operation,
            test,
            n_inspected: 0,
        })
    }

    let mut lines = input.lines();
    let mut monkeys = vec![];

    while let Some(monkey) = parse_monkey(&mut lines) {
        monkeys.push(monkey);
    }

    monkeys
}

fn borrow_all<T>(v: &mut Vec<T>) -> Vec<*mut T> {
    let v_raw = v.as_mut_ptr();
    let mut borrowed = vec![];

    unsafe {
        for i in 0..v.len() {
            borrowed.push(v_raw.add(i));
        }
    }

    borrowed
}

pub fn part_one(input: &str) -> Option<u64> {
    fn one_round(monkeys: &Vec<*mut Monkey>) {
        unsafe {
            for i in 0..monkeys.len() {
                let m = monkeys[i];
                while !(*m).items.is_empty() {
                    let mut item = (*m).items.remove(0);
                    item = (*m).operation.apply(item);
                    (*m).n_inspected += 1;

                    item /= 3;

                    let throw_to = (*m).test.check(item);
                    let throw_to = monkeys[throw_to];
                    (*throw_to).items.push(item);
                }
            }
        }
    }

    let mut monkeys = parse_input(input);
    let monkeys = borrow_all(&mut monkeys);

    for _ in 0..20 {
        one_round(&monkeys);
    }

    let n_inspected_desc = unsafe {
        monkeys
            .into_iter()
            .map(|m| (*m).n_inspected)
            .sorted()
            .rev()
            .collect_vec()
    };

    Some(n_inspected_desc[0] * n_inspected_desc[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    fn one_round(monkeys: &Vec<*mut Monkey>) {
        unsafe {
            let mod_by: u64 = monkeys.iter().map(|&m| (*m).test.div_by).product();

            for i in 0..monkeys.len() {
                let m = monkeys[i];
                while !(*m).items.is_empty() {
                    let mut item = (*m).items.remove(0);
                    item %= mod_by;
                    item = (*m).operation.apply(item);
                    (*m).n_inspected += 1;

                    let throw_to = (*m).test.check(item);
                    let throw_to = monkeys[throw_to];
                    (*throw_to).items.push(item);
                }
            }
        }
    }

    let mut monkeys = parse_input(input);
    let monkeys = borrow_all(&mut monkeys);

    for _ in 0..10000 {
        one_round(&monkeys);
    }

    let n_inspected_desc = unsafe {
        monkeys
            .into_iter()
            .map(|m| (*m).n_inspected)
            .sorted()
            .rev()
            .collect_vec()
    };

    Some(n_inspected_desc[0] * n_inspected_desc[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
