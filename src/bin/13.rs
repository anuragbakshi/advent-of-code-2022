use std::*;

#[derive(Debug, PartialEq)]
enum Elt {
    Int(u32),
    List(Vec<Elt>),
}

impl PartialOrd for Elt {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;

        match (self, other) {
            (Int(a), Int(b)) => a.partial_cmp(b),
            (List(a), List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (None, None) => return Some(Equal),
                        (None, Some(_)) => return Some(Less),
                        (Some(_), None) => return Some(Greater),
                        (Some(a), Some(b)) => match a.partial_cmp(b).unwrap() {
                            Equal => (),
                            res => return Some(res),
                        },
                    }
                }
            }
            (Int(a), List(_)) => List(vec![Int(*a)]).partial_cmp(other),
            (List(_), Int(b)) => self.partial_cmp(&List(vec![Int(*b)])),
        }
    }
}

// fn in_right_order(a: &Elt, b: &Elt) -> Option<bool> {
//     match (a, b) {
//         (Int(a), Int(b)) => {
//             if a == b {
//                 None
//             } else {
//                 Some(a < b)
//             }
//         }
//         (List(a), List(b)) => {
//             let mut a = a.iter();
//             let mut b = b.iter();

//             loop {
//                 match (a.next(), b.next()) {
//                     (None, None) => return None,
//                     (None, Some(_)) => return Some(true),
//                     (Some(_), None) => return Some(false),
//                     (Some(a), Some(b)) => match in_right_order(a, b) {
//                         None => (),
//                         Some(ans) => return Some(ans),
//                     },
//                 }
//             }
//         }
//         (Int(a), List(_)) => return in_right_order(&List(vec![Int(*a)]), b),
//         (List(_), Int(b)) => return in_right_order(a, &List(vec![Int(*b)])),
//     }
// }

use itertools::Itertools;
use Elt::*;

fn parse_list(chars: &mut str::Chars) -> Vec<Elt> {
    let mut l = vec![];

    loop {
        let next = chars.next().unwrap();
        match next.to_digit(10) {
            Some(n) => {
                l.push(Int(n));
            }
            None => {
                if next == ',' {
                } else if next == '[' {
                    l.push(List(parse_list(chars)));
                } else {
                    assert_eq!(next, ']');
                    return l;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Elt, Elt)> {
    let mut lines = input.lines();
    let mut pairs = vec![];

    while let Some((a, b)) = lines.next_tuple() {
        let mut a_chars = a.chars();
        let mut b_chars = b.chars();

        lines.next();

        assert_eq!(a_chars.next().unwrap(), '[');
        assert_eq!(b_chars.next().unwrap(), '[');

        let a = parse_list(&mut a_chars);
        let b = parse_list(&mut b_chars);

        pairs.push((List(a), List(b)));
    }

    pairs
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    let mut total = 0;

    for (i, (a, b)) in pairs.iter().enumerate() {
        if a < b {
            // println!("{a:?} < {b:?}; i = {}", i + 1);
            total += i + 1;
        }
        // if in_right_order(a, b).unwrap() {
        //     // println!("{a:?} < {b:?}; i = {}", i + 1);
        //     total += i + 1;
        // }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
