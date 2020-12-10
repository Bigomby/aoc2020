use std::{error::Error, ops::BitXor};

use crate::decode_lines;

pub fn solve_part1<'a, I: Into<&'a str>>(input: I) -> i32 {
    decode_lines(input, |s| parse_entry(s).unwrap())
        .filter(|&(min, max, target, data)| verify_part1(&min, &max, &target, data))
        .count() as i32
}

pub fn solve_part2<'a, I: Into<&'a str>>(input: I) -> i32 {
    decode_lines(input, |s| parse_entry(s).unwrap())
        .filter(|&(min, max, target, data)| verify_part2(&min, &max, &target, data))
        .count() as i32
}

fn verify_part1(min: &usize, max: &usize, target: &char, data: &str) -> bool {
    let count = data.matches(*target).count();

    count >= *min && count <= *max
}

fn verify_part2(a: &usize, b: &usize, target: &char, data: &str) -> bool {
    let char_a = data.chars().nth(a - 1).unwrap();
    let char_b = data.chars().nth(b - 1).unwrap();

    let char_a_matches = char_a.eq(target);
    let char_b_matches = char_b.eq(target);

    BitXor::bitxor(char_a_matches, char_b_matches)
}

fn parse_entry(raw: &str) -> Result<(usize, usize, char, &str), Box<dyn Error>> {
    let entry: Vec<&str> = raw.split(':').collect();

    let data = entry.get(1).ok_or("Cannot parse entry data")?.trim();

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
2-9 c: ccccccccc";
        let solution = solve_part1(input);

        assert_eq!(solution, 2);
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("inputs/puzzle02.input").unwrap();
        let solution = solve_part1(input.as_str());

        assert_eq!(solution, 477);
    }

    #[test]
    fn part2_example() {
        let input = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let solution = solve_part2(input);

        assert_eq!(solution, 1);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("inputs/puzzle02.input").unwrap();
        let solution = solve_part2(input.as_str());

        assert_eq!(solution, 686);
    }
}
