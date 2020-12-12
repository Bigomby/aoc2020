use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{BufRead, Lines},
};

use crate::concat_vec;

#[derive(Debug, Clone)]
struct RVec<T>(T, Vec<RVec<T>>);

fn solve_part1<T: BufRead>(input: Lines<T>) -> i32 {
    let parent_re = Regex::new(r"(.+)\sbags\scontain\s(.*)\.").unwrap();
    let content_re = Regex::new(r"(\d+)\s(.+?)\sbags?").unwrap();

    let db = input.fold(HashMap::new(), |db, line| {
        build_db_rev(db, &parent_re, &content_re, &line.unwrap())
    });
    let parents = trace_path(&db, ("shiny gold", 0));
    let uniques = flatten_db(&parents)
        .iter()
        .map(|(color, _)| color)
        .unique()
        .count();
    let result = uniques - 1;

    result as i32
}

fn solve_part2<T: BufRead>(input: Lines<T>) -> i32 {
    let parent_re = Regex::new(r"(.+)\sbags\scontain\s(.*)\.").unwrap();
    let content_re = Regex::new(r"(\d+)\s(.+?)\sbags?").unwrap();

    let db = input.fold(HashMap::new(), |db, line| {
        build_db(db, &parent_re, &content_re, &line.unwrap())
    });

    let parents = trace_path(&db, ("shiny gold", 1));
    let total = compute_total(&parents) - 1;

    total as i32
}

fn trace_path<'a>(
    db: &'a HashMap<String, Vec<(String, usize)>>,
    item: (&'a str, usize),
) -> RVec<(&'a str, usize)> {
    match db.get(item.0) {
        None => RVec(item, Vec::new()),
        Some(xs) => RVec(
            item,
            xs.iter()
                .map(|(item, amount)| trace_path(db, (&item, *amount)))
                .collect(),
        ),
    }
}

fn flatten_db<'a>(ps: &'a RVec<(&str, usize)>) -> Vec<(&'a str, usize)> {
    match &ps.1[..] {
        [] => vec![ps.0],
        _ => {
            ps.1.iter()
                .fold(vec![ps.0], |res, ps| {
                    concat_vec(
                        flatten_db(ps),
                        res.iter()
                            .map(|(color, amount)| (*color, amount * ps.0 .1))
                            .collect(),
                    )
                    .map(|(a, b)| (a, b))
                    .collect()
                })
                .into_iter()
                .map(|(a, b)| (a, b))
                .collect()
        }
    }
}

fn build_db(
    db: HashMap<String, Vec<(String, usize)>>,
    parent_re: &Regex,
    content_re: &Regex,
    line: &str,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut db = db.clone();

    for cap in parent_re.captures_iter(line) {
        let parent_color = cap[1].to_string();

        for bag in content_re.captures_iter(line) {
            let amount = bag[1].parse::<usize>().unwrap();
            let color = bag[2].to_string();
            let val = db.entry(parent_color.clone()).or_insert(Vec::new());

            val.push((color, amount));
        }
    }

    db
}

fn build_db_rev(
    db: HashMap<String, Vec<(String, usize)>>,
    parent_re: &Regex,
    content_re: &Regex,
    line: &str,
) -> HashMap<String, Vec<(String, usize)>> {
    let mut db = db.clone();

    for cap in parent_re.captures_iter(line) {
        let parent_color = cap[1].to_string();

        for bag in content_re.captures_iter(line) {
            let amount = bag[1].parse::<usize>().unwrap();
            let color = bag[2].to_string();
            let val = db.entry(color).or_insert(Vec::new());

            val.push((parent_color.clone(), amount));
        }
    }

    db
}

fn compute_total(db: &RVec<(&str, usize)>) -> usize {
    let RVec((_, amount), content) = db;

    match &content[..] {
        [] => *amount,
        _ => content.iter().map(|x| compute_total(x)).sum::<usize>() * amount + amount,
    }
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
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let line_reader = Cursor::new(content).lines();

        let solution = solve_part1(line_reader);

        assert_eq!(solution, 4)
    }

    #[test]
    fn part1_input() {
        let content = File::open("inputs/puzzle07.input").unwrap();
        let line_reader = BufReader::new(content).lines();

        let solution = solve_part1(line_reader);

        assert_eq!(solution, 177)
    }

    #[test]
    fn part2_example() {
        let content = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let line_reader = Cursor::new(content).lines();

        let solution = solve_part2(line_reader);

        assert_eq!(solution, 32)
    }

    #[test]
    fn part2_input() {
        let content = File::open("inputs/puzzle07.input").unwrap();
        let line_reader = BufReader::new(content).lines();

        let solution = solve_part2(line_reader);

        assert_eq!(solution, 34988)
    }
}
