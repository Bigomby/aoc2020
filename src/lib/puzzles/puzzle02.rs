use std::{error::Error, ops::BitXor};

use super::Puzzle;

pub struct Puzzle02 {
    input: String,
}

impl Puzzle<(i32, i32)> for Puzzle02 {
    fn build(input: String) -> Self {
        Self { input }
    }

    fn solve(&self) -> (i32, i32) {
        let part1 = self
            .input
            .split('\n')
            .into_iter()
            .map(|entry| Part1Entry::from_raw(&entry))
            .filter(|entry| entry.verify())
            .count() as i32;

        let part2 = self
            .input
            .split('\n')
            .into_iter()
            .map(|entry| Part2Entry::from_raw(&entry))
            .filter(|entry| entry.verify())
            .count() as i32;

        (part1, part2)
    }
}

trait Entry {
    fn verify(&self) -> bool;
}

struct Part1Entry {
    min: usize,
    max: usize,
    target: char,
    data: String,
}

impl Part1Entry {
    fn from_raw(raw: &str) -> Self {
        let (min, max, target, data) = parse_entry(raw).unwrap();

        Part1Entry {
            min,
            max,
            target,
            data,
        }
    }
}

impl Entry for Part1Entry {
    fn verify(&self) -> bool {
        let count = self.data.matches(self.target).count();
        count >= self.min && count <= self.max
    }
}

struct Part2Entry {
    a: usize,
    b: usize,
    target: char,
    data: String,
}

impl Part2Entry {
    fn from_raw(raw: &str) -> Self {
        let (a, b, target, data) = parse_entry(raw).unwrap();

        Part2Entry { a, b, target, data }
    }
}

impl Entry for Part2Entry {
    fn verify(&self) -> bool {
        let char_a = self.data.chars().nth(self.a - 1).unwrap();
        let char_b = self.data.chars().nth(self.b - 1).unwrap();

        let char_a_matches = char_a.eq(&self.target);
        let char_b_matches = char_b.eq(&self.target);

        BitXor::bitxor(char_a_matches, char_b_matches)
    }
}

fn parse_entry(raw: &str) -> Result<(usize, usize, char, String), Box<dyn Error>> {
    let entry: Vec<&str> = raw.split(':').collect();

    let data = entry
        .get(1)
        .ok_or("Cannot parse entry data")?
        .trim()
        .to_string();

    let params: Vec<&str> = entry
        .get(0)
        .ok_or("Cannot parse entry parameters")?
        .split(' ')
        .collect();

    let min_max: Vec<&str> = params
        .get(0)
        .ok_or("Cannot parse entry min and max")?
        .split('-')
        .collect();

    let character = params
        .get(1)
        .ok_or("Cannot parse entry character")?
        .chars()
        .nth(0)
        .unwrap();

    let a = min_max.get(0).ok_or("Cannot parse entry min")?.parse()?;
    let b = min_max.get(1).ok_or("Cannot parse entry max")?.parse()?;

    Ok((a, b, character, data))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            .to_string();

        let puzzle = Puzzle02::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution.0, 2);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("inputs/puzzle02.input").unwrap();
        let puzzle = Puzzle02::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution.0, 477);
    }

    #[test]
    fn part2_example() {
        let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            .to_string();

        let puzzle = Puzzle02::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution.1, 1);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("inputs/puzzle02.input").unwrap();
        let puzzle = Puzzle02::build(input);
        let solution = puzzle.solve();

        assert_eq!(solution.1, 686);
    }
}
