use crate::take_half;

use super::Puzzle;

use itertools::Itertools;

struct Puzzle05 {
    entries: Vec<(String, String)>,
}

impl Puzzle<(i32, i32)> for Puzzle05 {
    fn build(input: String) -> Self {
        let entries: Vec<(String, String)> = input
            .split('\n')
            .into_iter()
            .map(|entry| (entry.split_at(7)))
            .map(|(row, col)| (row.to_string(), col.to_string()))
            .collect();

        Puzzle05 { entries }
    }

    fn solve(&self) -> (i32, i32) {
        (self.solve_part1(), self.solve_part2())
    }
}

impl Puzzle05 {
    fn solve_part1(&self) -> i32 {
        *Self::compute_ids(&self.entries).iter().max().unwrap()
    }

    fn solve_part2(&self) -> i32 {
        let seat_ids = Self::compute_ids(&self.entries);
        let sorted_seat_ids: Vec<&i32> = seat_ids.iter().sorted().collect();

        let seat: Vec<i32> = sorted_seat_ids
            .into_iter()
            .tuple_windows()
            .map(|(p, c)| (c, c - p))
            .filter_map(|(s, d)| Self::by_distance(*s, d))
            .collect::<Vec<i32>>();

        *seat.first().unwrap()
    }

    fn by_distance(seat: i32, distance: i32) -> Option<i32> {
        if distance >= 2 {
            Some(seat - (distance - 1))
        } else {
            None
        }
    }

    fn compute_ids(entries: &[(String, String)]) -> Vec<i32> {
        entries
            .iter()
            .map(|(row, col)| {
                let (row, _) = row.chars().fold((0, 127), |r, s| take_half(r, s));
                let (col, _) = col.chars().fold((0, 7), |r, s| take_half(r, s));
                (row, col)
            })
            .map(|(row, col)| row * 8 + col)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let input = "FBFBBFFRLR".to_string();
        let puzzle = Puzzle05::build(input);
        let solution = puzzle.solve_part1();

        assert_eq!(solution, 357)
    }

    #[test]
    fn example2() {
        let input = "BFFFBBFRRR".to_string();
        let puzzle = Puzzle05::build(input);
        let solution = puzzle.solve_part1();

        assert_eq!(solution, 567)
    }

    #[test]
    fn example3() {
        let input = "FFFBBBFRRR".to_string();
        let puzzle = Puzzle05::build(input);
        let solution = puzzle.solve_part1();

        assert_eq!(solution, 119)
    }

    #[test]
    fn example4() {
        let input = "BBFFBBFRLL".to_string();
        let puzzle = Puzzle05::build(input);
        let solution = puzzle.solve_part1();

        assert_eq!(solution, 820)
    }

    #[test]
    fn input() {
        let input = fs::read_to_string("inputs/puzzle05.input").unwrap();
        let puzzle = Puzzle05::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution, (955, 569))
    }
}
