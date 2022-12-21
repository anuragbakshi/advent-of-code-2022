use itertools::Itertools;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect_vec()
}

fn wrapped_index<T>(v: &Vec<T>, i: i64) -> usize {
    let mut new_index = i as i64;
    new_index %= v.len() as i64;
    new_index += v.len() as i64;
    new_index %= v.len() as i64;

    new_index as usize
}

fn mix(input: &Vec<i64>, key: i64, n: u64) -> i64 {
    let mut with_index = input.iter().map(|x| x * key).enumerate().collect_vec();

    for _ in 0..n {
        for i in 0..with_index.len() {
            let index_to_move = with_index.iter().position(|(ord, _)| *ord == i).unwrap();
            let (ord, x) = with_index.remove(index_to_move);

            let new_index = wrapped_index(&with_index, (index_to_move as i64) + x);

            with_index.insert(new_index as usize, (ord, x));
        }
    }

    let index_of_0 = with_index.iter().position(|(_, x)| *x == 0).unwrap() as i64;
    let i1000 = wrapped_index(&with_index, index_of_0 + 1000);
    let i2000 = wrapped_index(&with_index, index_of_0 + 2000);
    let i3000 = wrapped_index(&with_index, index_of_0 + 3000);

    with_index[i1000].1 + with_index[i2000].1 + with_index[i3000].1
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse_input(input);

    Some(mix(&input, 1, 1))
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse_input(input);

    Some(mix(&input, 811589153, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
