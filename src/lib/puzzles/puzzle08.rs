use itertools::Itertools;
use std::{
    convert::{TryFrom, TryInto},
    io::{BufRead, Lines},
};

use crate::PuzzleError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Noop(i32),
}

impl TryFrom<String> for Instruction {
    type Error = PuzzleError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.chars().take(3).collect::<String>().as_str() {
            "acc" => Ok(Self::Acc(parse_arg(&s).ok_or(PuzzleError {})?)),
            "jmp" => Ok(Self::Jmp(parse_arg(&s).ok_or(PuzzleError {})?)),
            "nop" => Ok(Self::Noop(parse_arg(&s).ok_or(PuzzleError {})?)),
            _ => Err(PuzzleError {}),
        }
    }
}

fn solve_part1<T: BufRead>(input: Lines<T>) -> i32 {
    let instructions: Vec<Instruction> = input.map(|a| a.unwrap().try_into().unwrap()).collect();
    let mut acc: i32 = 0;
    let mut pc: usize = 0;
    let mut pc_registry: Vec<usize> = Vec::new();

    while !pc_registry.contains(&pc) {
        pc_registry.push(pc);
        (pc, acc) = execute_instruction(pc, acc, instructions.get(pc).unwrap());
    }

    acc
}

fn solve_part2<T: BufRead>(input: Lines<T>) -> i32 {
    let instructions: Vec<Instruction> = input.map(|a| a.unwrap().try_into().unwrap()).collect();
    let last_pc = instructions.len();

    let jmps: Vec<(usize, &Instruction)> = instructions
        .iter()
        .enumerate()
        .filter(|a| matches!(a.1, Instruction::Jmp(_)))
        .collect();

    let noops: Vec<(usize, &Instruction)> = instructions
        .iter()
        .enumerate()
        .filter(|a| matches!(a.1, Instruction::Noop(_)))
        .collect();

    let swaps: Vec<(usize, usize)> = jmps
        .iter()
        .cartesian_product(noops.iter())
        .map(|(a, b)| (a.0, b.0))
        .collect();

    let cosa: Vec<(usize, i32, Vec<usize>)> = swaps
        .into_iter()
        .filter_map(|(a, b)| {
            let instructions = swap_instructions(&instructions, a, b);
            let mut acc: i32 = 0;
            let mut pc: usize = 0;
            let mut pc_registry: Vec<usize> = Vec::new();

            loop {
                if pc_registry.contains(&pc) {
                    break None;
                }
                pc_registry.push(pc);

                if pc >= last_pc {
                    break Some((pc, acc, pc_registry));
                }

                (pc, acc) = execute_instruction(pc, acc, instructions.get(pc).unwrap());
            }
        })
        .take(1)
        .collect();

    let (_pc, acc, _pc_registry) = cosa.get(0).expect("Solution not found");

    *acc
}

fn execute_instruction(pc: usize, acc: i32, instruction: &Instruction) -> (usize, i32) {
    let mut pc = pc;
    let mut acc = acc;

    match instruction {
        Instruction::Acc(n) => {
            acc += n;
            pc += 1;
        }
        Instruction::Jmp(n) => {
            pc = ((pc as i32) + n) as usize;
        }
        Instruction::Noop(_) => {
            pc += 1;
        }
    };

    (pc, acc)
}

fn parse_arg(s: &str) -> Option<i32> {
    match &s[4..5] {
        "+" => Some(s[5..].parse::<i32>().unwrap()),
        "-" => Some(-s[5..].parse::<i32>().unwrap()),
        _ => None,
    }
}

fn swap_instructions(instructions: &Vec<Instruction>, a: usize, b: usize) -> Vec<Instruction> {
    let mut instructions = instructions.clone();

    let instruction_a = *instructions.get(a).unwrap();
    let instruction_b = *instructions.get(b).unwrap();

    match instruction_a {
        Instruction::Jmp(n) => match instruction_b {
            Instruction::Noop(_) => instructions[a] = Instruction::Noop(n),
            _ => {}
        },
        Instruction::Noop(n) => match instruction_b {
            Instruction::Jmp(_) => instructions[a] = Instruction::Noop(n),
            _ => {}
        },
        _ => {}
    };

    match instruction_b {
        Instruction::Jmp(n) => match instruction_b {
            Instruction::Noop(_) => instructions[b] = Instruction::Noop(n),
            _ => {}
        },
        Instruction::Noop(n) => match instruction_b {
            Instruction::Jmp(_) => instructions[b] = Instruction::Noop(n),
            _ => {}
        },
        _ => {}
    };

    instructions
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
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let line_reader = Cursor::new(content).lines();

        let solution = solve_part1(line_reader);

        assert_eq!(solution, 5)
    }

    #[test]
    fn part1_input() {
        let content = File::open("inputs/puzzle08.input").unwrap();
        let line_reader = BufReader::new(content).lines();

        let solution = solve_part1(line_reader);

        assert_eq!(solution, 1487)
    }

    #[test]
    fn part2_example() {
        let content = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let line_reader = Cursor::new(content).lines();

        let solution = solve_part2(line_reader);

        assert_eq!(solution, 8)
    }

    #[test]
    fn part2_input() {
        let content = File::open("inputs/puzzle08.input").unwrap();
        let line_reader = BufReader::new(content).lines();

        let solution = solve_part2(line_reader);

        assert_eq!(solution, 1607)
    }
}
