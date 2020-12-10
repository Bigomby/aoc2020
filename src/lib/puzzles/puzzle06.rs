use itertools::Itertools;

use crate::{decode_lines_group, frequencies};

fn solve_part1<'a, I: Into<&'a str>>(input: I) -> i32 {
    decode_lines_group(input, |s| s.chars().filter(|&c| c != '\n').unique().count()).sum::<usize>()
        as i32
}

fn solve_part2<'a, I: Into<&'a str>>(input: I) -> i32 {
    decode_lines_group(input, |group| (group, group.split("\n").count()))
        .map(|(group, group_size)| {
            frequencies(group)
                .filter(|(c, count)| *c != '\n' && *count == group_size)
                .count()
        })
        .sum::<usize>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_example() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        let solution = solve_part1(input);

        assert_eq!(solution, 11)
    }

    #[test]
    fn part1_input() {
        let input = fs::read_to_string("inputs/puzzle06.input").unwrap();
        let solution = solve_part1(input.as_str());

        assert_eq!(solution, 6714)
    }

    #[test]
    fn part2_example() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

        let solution = solve_part2(input);

        assert_eq!(solution, 6)
    }

    #[test]
    fn part2_input() {
        let input = fs::read_to_string("inputs/puzzle06.input").unwrap();
        let solution = solve_part2(input.as_str());

        assert_eq!(solution, 3435)
    }
}
