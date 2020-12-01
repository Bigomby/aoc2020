use aoc2020::{check_sum_equals, compute_product, self_cross_product};

#[allow(dead_code)] // Allow dead until CLI is ready
pub fn solve_puzzle(input: &[i32]) -> (i64, i64) {
    let result_part_1 = solve_part(input, 2020, 2).expect("Solution not found");
    let result_part_2 = solve_part(input, 2020, 3).expect("Solution not found");

    (result_part_1, result_part_2)
}

#[allow(dead_code)] // Allow dead until CLI is ready
fn solve_part(input: &[i32], target: i32, dims: usize) -> Option<i64> {
    let result: Vec<i64> = self_cross_product(input, dims)
        .filter(|xs| check_sum_equals(&xs, target))
        .take(1)
        .map(|xs| compute_product(&xs))
        .collect();

    match result.get(0) {
        Some(x) => Some(*x),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{self, BufRead},
    };

    #[test]
    fn part1() {
        let file = File::open("inputs/puzzle01.input").unwrap();
        let lines = io::BufReader::new(file).lines();

        let mut input: Vec<i32> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let entry: i32 = line.parse().unwrap();

            input.push(entry);
        }

        let solution = solve_part(&input, 2020, 2).unwrap();

        assert_eq!(solution, 989824);
    }

    #[test]
    fn part2() {
        let file = File::open("inputs/puzzle01.input").unwrap();
        let lines = io::BufReader::new(file).lines();

        let mut input: Vec<i32> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let entry: i32 = line.parse().unwrap();

            input.push(entry);
        }

        let solution = solve_part(&input, 2020, 3).unwrap();

        assert_eq!(solution, 66432240);
    }
}
