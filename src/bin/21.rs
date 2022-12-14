use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    fn invert(&self) -> Operator {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
        }
    }
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
        }
        .to_owned()
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(format!("bad operator {s}").to_owned()),
        }
    }
}

#[derive(Debug)]
enum Value<'a> {
    Const(i64),
    Monkey(&'a str),
}

impl Value<'_> {
    fn maybe_resolve(&mut self, resolved: (&str, i64)) {
        match self {
            Self::Const(_) => {}
            &mut Self::Monkey(name) => {
                if resolved.0 == name {
                    *self = Self::Const(resolved.1);
                }
            }
        }
    }
}

impl ToString for Value<'_> {
    fn to_string(&self) -> String {
        match self {
            Self::Const(c) => c.to_string(),
            &Self::Monkey(name) => name.to_owned(),
        }
    }
}

#[derive(Debug)]
enum Equation<'a> {
    Const(i64),
    Formula(Value<'a>, Operator, Value<'a>),
}

impl Equation<'_> {
    fn maybe_simplify(&mut self, resolved: (&str, i64)) {
        match self {
            Self::Const(_) => {}
            Self::Formula(lhs, op, rhs) => {
                lhs.maybe_resolve(resolved);
                rhs.maybe_resolve(resolved);

                if let (Value::Const(lhs), Value::Const(rhs)) = (lhs, rhs) {
                    *self = Self::Const(op.apply(*lhs, *rhs));
                }
            }
        }
    }

    fn invert<'a>(&'a self, equal_to: &'a str) -> (&'a str, Equation<'a>) {
        // a = 1 + b
        // a = 1 * b
        // a = 1 - b
        // a = 1 / b

        // a - 1 = b
        // a / 1 = b
        // b = 1 - a
        // b = 1 / a

        // a = b + 1
        // a = b * 1
        // a = b - 1
        // a = b / 1

        // a - 1 = b
        // a / 1 = b
        // a + 1 = b
        // a * 1 = b

        use Operator::*;
        use Value::*;
        match self {
            &Equation::Formula(Const(c), op, Monkey(m)) => {
                let inv = match op {
                    Add | Mul => Equation::Formula(Monkey(equal_to), op.invert(), Const(c)),
                    Sub | Div => Equation::Formula(Const(c), op, Monkey(equal_to)),
                };
                (m, inv)
            }
            &Equation::Formula(Monkey(m), op, Const(c)) => (
                m,
                Equation::Formula(Monkey(equal_to), op.invert(), Const(c)),
            ),
            _ => panic!(),
        }
    }
}

impl ToString for Equation<'_> {
    fn to_string(&self) -> String {
        match self {
            Self::Const(c) => c.to_string(),
            Self::Formula(lhs, op, rhs) => {
                format!("{} {} {}", lhs.to_string(), op.to_string(), rhs.to_string())
            }
        }
    }
}

fn print_monkeys(monkeys: &HashMap<&str, Equation>) {
    for (name, eq) in monkeys.iter() {
        println!("{} => {}", name, eq.to_string());
    }
}

fn parse_input(input: &str) -> HashMap<&str, Equation> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let name = &line[..4];

        let eq = &line[6..].split_whitespace().collect_vec();
        let eq = if eq.len() == 1 {
            Equation::Const(eq[0].parse().unwrap())
        } else {
            Equation::Formula(
                Value::Monkey(eq[0]),
                eq[1].parse().unwrap(),
                Value::Monkey(eq[2]),
            )
        };

        monkeys.insert(name, eq);
    }

    monkeys
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = parse_input(input);

    loop {
        if let Equation::Const(ans) = monkeys["root"] {
            return Some(ans);
        }

        let next_const = monkeys
            .iter()
            .find_map(|(name, eq)| match eq {
                Equation::Const(v) => Some((name, v)),
                _ => None,
            })
            .unwrap();

        let next_const = (*next_const.0, *next_const.1);

        monkeys.remove(next_const.0);

        for m in monkeys.iter_mut() {
            m.1.maybe_simplify(next_const);
        }
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = parse_input(input);
    monkeys.remove("humn");

    loop {
        let next_const = monkeys.iter().find_map(|(name, eq)| match eq {
            Equation::Const(v) => Some((name, v)),
            _ => None,
        });

        match next_const {
            None => break,
            Some(next_const) => {
                let next_const = (*next_const.0, *next_const.1);

                monkeys.remove(next_const.0);

                for m in monkeys.iter_mut() {
                    m.1.maybe_simplify(next_const);
                }
            }
        }
    }

    let (mut value, solve_for) = match monkeys["root"] {
        Equation::Formula(Value::Const(c), _, Value::Monkey(m))
        | Equation::Formula(Value::Monkey(m), _, Value::Const(c)) => (c, m),
        _ => panic!(),
    };

    let mut solve_for = solve_for.to_owned();

    loop {
        if solve_for == "humn" {
            return Some(value);
        }

        let eq = monkeys.remove(solve_for.as_str()).unwrap();
        let (unknown, mut eq) = eq.invert(solve_for.as_str());
        eq.maybe_simplify((solve_for.as_str(), value));

        match eq {
            Equation::Const(c) => {
                value = c;
                solve_for = unknown.to_owned();
            }
            _ => panic!(),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
