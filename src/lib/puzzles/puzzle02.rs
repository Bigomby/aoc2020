use std::{error::Error, ops::BitXor};

pub struct Puzzle02 {}

impl Puzzle02 {
    pub fn solve_part1(input: Vec<String>) -> i32 {
        input
            .into_iter()
            .map(|entry| Part1Entry::from_raw(&entry))
            .filter(|entry| entry.verify())
            .count() as i32
    }

    pub fn solve_part2(input: Vec<String>) -> i32 {
        input
            .into_iter()
            .map(|entry| Part2Entry::from_raw(&entry))
            .filter(|entry| entry.verify())
            .count() as i32
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
    use super::*;
    use std::{
        fs::File,
        io::{self, BufRead},
    };

    #[test]
    fn part1_example() {
        let input = "\
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";
        let input = input
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let solution = Puzzle02::solve_part1(input);

        assert_eq!(solution, 2);
    }

    #[test]
    fn part1() {
        let file = File::open("inputs/puzzle02.input").unwrap();

        let mut input: Vec<String> = Vec::new();
        for line in io::BufReader::new(file).lines() {
            input.push(line.unwrap());
        }

        let solution = Puzzle02::solve_part1(input.into());

        assert_eq!(solution, 477);
    }

    #[test]
    fn part2_example() {
        let input = "\
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc";
        let input = input
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let solution = Puzzle02::solve_part2(input);

        assert_eq!(solution, 1);
    }

    #[test]
    fn part2() {
        let file = File::open("inputs/puzzle02.input").unwrap();

        let mut input: Vec<String> = Vec::new();
        for line in io::BufReader::new(file).lines() {
            input.push(line.unwrap());
        }

        let solution = Puzzle02::solve_part2(input.into());

        assert_eq!(solution, 686);
    }
}
