use std::cmp::max;

use itertools::Itertools;
use ndarray::*;

type Grid = Array2<u32>;

fn parse_input(input: &str) -> Grid {
    let v = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    Grid::from_shape_vec((v.len(), v[0].len()), v.concat()).unwrap()
}

// fn at(grid: &Grid, r: i32, c: i32) -> u32 {
//     if r < 0 || (r as usize) >= grid.len() || c < 0 || (c as usize) >= grid[0].len() {
//         0
//     } else {
//         grid[r as usize][c as usize]
//     }
// }

fn mark_visible(view: ArrayView1<'_, u32>, mut visible: ArrayViewMut1<'_, u32>) {
    let mut highest = -1i32;

    for (i, &x) in view.into_iter().enumerate() {
        let x = x as i32;
        if x > highest {
            visible[i] = 1;
            highest = x;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut visible = Array::zeros(grid.raw_dim());

    for r in 0..grid.len_of(Axis(0)) {
        let s = s![r, ..];
        let s_rev = s![r, ..;-1];

        mark_visible(grid.slice(s), visible.slice_mut(s));
        mark_visible(grid.slice(s_rev), visible.slice_mut(s_rev));
    }

    for c in 0..grid.len_of(Axis(1)) {
        let s = s![.., c];
        let s_rev = s![..;-1, c];

        mark_visible(grid.slice(s), visible.slice_mut(s));
        mark_visible(grid.slice(s_rev), visible.slice_mut(s_rev));
    }

    Some(visible.sum())
}

fn view_score_along_slice(view: ArrayView1<'_, u32>, mut score: ArrayViewMut1<'_, u32>) {
    let mut last_tree_of_height = Array::zeros(10);

    for (i, &h) in view.into_iter().enumerate() {
        let h = h as usize;

        let &can_see_until = last_tree_of_height
            .slice(s![h..])
            .into_iter()
            .max()
            .unwrap();

        score[i] = (i - can_see_until) as u32;

        last_tree_of_height[h] = i;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut score_r_p = Array::zeros(grid.raw_dim());
    let mut score_r_n = Array::zeros(grid.raw_dim());
    let mut score_c_p = Array::zeros(grid.raw_dim());
    let mut score_c_n = Array::zeros(grid.raw_dim());

    for r in 0..grid.len_of(Axis(0)) {
        let s = s![r, ..];
        let s_rev = s![r, ..;-1];

        view_score_along_slice(grid.slice(s), score_r_p.slice_mut(s));
        view_score_along_slice(grid.slice(s_rev), score_r_n.slice_mut(s_rev));
    }

    for c in 0..grid.len_of(Axis(1)) {
        let s = s![.., c];
        let s_rev = s![..;-1, c];

        view_score_along_slice(grid.slice(s), score_c_p.slice_mut(s));
        view_score_along_slice(grid.slice(s_rev), score_c_n.slice_mut(s_rev));
    }

    let mut best = 0;
    for r in 0..grid.len_of(Axis(0)) {
        for c in 0..grid.len_of(Axis(1)) {
            best = max(
                best,
                score_r_p[[r, c]] * score_r_n[[r, c]] * score_c_p[[r, c]] * score_c_n[[r, c]],
            )
        }
    }

    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
