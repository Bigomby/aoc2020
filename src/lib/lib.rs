mod puzzles;

use itertools::Itertools;

pub fn self_cross_product(input: &[i32], dims: usize) -> impl Iterator<Item = Vec<i32>> + '_ {
    (0..dims)
        .map(|_| input.iter().cloned())
        .multi_cartesian_product()
}

pub fn compute_product(xs: &[i32]) -> i64 {
    xs.iter().map(|x| *x as i64).product::<i64>()
}

pub fn check_sum_equals(xs: &[i32], target: i32) -> bool {
    xs.iter().sum::<i32>() == target
}
