use itertools::Itertools;
use std::io::{BufRead, Lines};

fn solve_part1<T: BufRead>(input: Lines<T>, window_size: usize) -> i64 {
    let solution = input
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .windows(window_size + 1)
        .map(|xs| {
            (
                xs.last().unwrap(),
                xs.iter().take(window_size).collect::<Vec<&i64>>(),
            )
        })
        .filter(|(n, window)| {
            window
                .iter()
                .enumerate()
                .cartesian_product(window.iter().enumerate())
                .take(window.len().pow(2))
                .filter_map(
                    |((i1, n1), (i2, n2))| {
                        if i1 != i2 {
                            Some(*n1 + *n2)
                        } else {
                            None
                        }
                    },
                )
                .all(|sum| sum != **n)
        })
        .map(|(n, _)| *n)
        .take(1)
        .collect::<Vec<i64>>();

    *solution.first().expect("Solution not found")
}

fn solve_part2<T: BufRead>(input: Lines<T>, window_size: usize) -> i64 {
    let data = input
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part1 = data
        .windows(window_size + 1)
        .map(|xs| {
            (
                xs.last().unwrap(),
                xs.iter().take(window_size).collect::<Vec<&i64>>(),
            )
        })
        .filter(|(n, window)| {
            window
                .iter()
                .enumerate()
                .cartesian_product(window.iter().enumerate())
                .take(window.len().pow(2))
                .filter_map(
                    |((i1, n1), (i2, n2))| {
                        if i1 != i2 {
                            Some(*n1 + *n2)
                        } else {
                            None
                        }
                    },
                )
                .all(|sum| sum != **n)
        })
        .map(|(n, _)| *n)
        .take(1)
        .map(|a| a)
        .collect::<Vec<i64>>();

    let invalid_number = *part1.first().unwrap();

    let result: Vec<i64> = (0..data.len())
        .filter_map(|i| compute_sums_list(&data, i, invalid_number))
        .map(|xs| xs.iter().min().unwrap() + xs.iter().max().unwrap())
        .take(1)
        .collect();

    *result.first().unwrap()
}

fn compute_sums_list(input: &Vec<i64>, start: usize, target: i64) -> Option<Vec<i64>> {
    let mut result = Vec::new();
    let mut sum = 0;

    for n in &input[start..] {
        result.push(*n);
        sum += n;

        if sum == target && result.len() > 1 {
            return Some(result);
        } else if sum > target {
            return None;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader, Cursor},
    };

    #[test]
    fn part1_example() {
        let content = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";
        let line_reader = Cursor::new(content).lines();
        let solution = solve_part1(line_reader, 5);

        assert_eq!(solution, 127)
    }

    #[test]
    fn part1_input() {
        let content = File::open("inputs/puzzle09.input").unwrap();
        let line_reader = BufReader::new(content).lines();
        let solution = solve_part1(line_reader, 25);

        assert_eq!(solution, 1930745883)
    }

    #[test]
    fn part2_example() {
        let content = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";
        let line_reader = Cursor::new(content).lines();
        let solution = solve_part2(line_reader, 5);

        assert_eq!(solution, 62)
    }

    #[test]
    fn part2_input() {
        let content = File::open("inputs/puzzle09.input").unwrap();
        let line_reader = BufReader::new(content).lines();
        let solution = solve_part2(line_reader, 25);

        assert_eq!(solution, 268878261)
    }
}
