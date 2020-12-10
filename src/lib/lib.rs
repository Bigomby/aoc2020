mod puzzles;

use itertools::Itertools;

pub fn decode_lines<'a, I, T, F>(input: I, decode: F) -> impl Iterator<Item = T> + 'a
where
    I: Into<&'a str>,
    F: Fn(&'a str) -> T + 'a,
{
    input.into().split('\n').map(move |a| decode(a))
}

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

pub fn take_half(range: (i32, i32), section: char) -> (i32, i32) {
    match section {
        'F' | 'L' => (range.0, range.0 + (range.1 - range.0) / 2),
        'B' | 'R' => (range.1 - (range.1 - range.0) / 2, range.1),
        _ => range,
    }
}
