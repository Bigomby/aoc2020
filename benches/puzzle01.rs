use std::{
    fs::File,
    io::{self, BufRead},
};

use aoc2020::{check_sum_equals, compute_product, self_cross_product};

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

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

fn criterion_benchmark(c: &mut Criterion) {
    let file = File::open("inputs/puzzle01.input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut input: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let entry: i32 = line.parse().unwrap();

        input.push(entry);
    }

    c.bench_function("find_index_where (dimension: 2)", |b| {
        b.iter(|| solve_part(&input, 2020, 2))
    });

    c.bench_function("find_index_where (dimension: 3)", |b| {
        b.iter(|| solve_part(&input, 2020, 3))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
