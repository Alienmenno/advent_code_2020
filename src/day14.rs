use std::fs;
use std::iter;
use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
    let instructions = parse_file("dat/day14.txt");
    // println!("Instructions: {:?}", instructions);

    let r = execute_init_program(&instructions);
    println!("result: {}", r);

    let r = execute_mem_decoder_init_program(&instructions);
    println!("result: {}", r);
}

#[derive(Debug, Clone, Copy,)]
struct WriteInstruction {
    address: u64,
    value: u64,
}

#[derive(Debug, Clone, Copy,)]
struct WriteMask {
    mask: u64,
    value: u64,
}

#[derive(Debug)]
enum Instruction {
    Write(WriteInstruction),
    Mask(WriteMask),
}

fn execute_init_program(instructions: &Vec<Instruction>) -> u64 {
    let mut cm = WriteMask{mask: 0, value: 0};
    let mut register: HashMap<u64, u64> = HashMap::new();
    instructions.iter().for_each(|i| {
        match i {
            Instruction::Write(w) => {register.insert(w.address, (w.value & cm.mask) | cm.value);},
            Instruction::Mask(m) => {cm = *m;},
        }
    });
    // println!("register: {:?}", register);
    return register.values().sum();
}

#[allow(unused_variables)]
fn execute_mem_decoder_init_program(instructions: &Vec<Instruction>) -> u64 {
    let mut cm = WriteMask{mask: 0, value: 0};
    let mut register: HashMap<u64, u64> = HashMap::new();
    instructions.iter().enumerate().for_each(|(i, instr)| {
        // println!("{}, {:?}", i, instr);
        match instr {
            Instruction::Write(w) => {
                let ones = cm.mask.count_ones() as usize;
                let masks = (0..=36).map(|i| cm.mask & (1 << i))
                                    .unique()
                                    .chain(iter::repeat(0).take(ones - 1))
                                    .combinations(ones)
                                    .map(|v| v.iter().fold(0, |acc, x| acc | x))
                                    // .inspect(|x| println!("m: {:#036b}", x))
                                    .collect::<Vec<u64>>();

                masks.iter().for_each(|m| {
                    let a = (w.address | cm.value) & !cm.mask | m;
                    // println!("adr: {:#036b}", a);
                    register.insert(a, w.value);
                });
            },
            Instruction::Mask(m) => {cm = *m;},
        }
    });
    // println!("register: {:?}", register);
    return register.values().sum();
}

fn parse_file(file_name: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    return input.lines().map(|l| {
        if l.starts_with("mask") {
            let mask_string = l.split_whitespace().last().unwrap();
            let m = mask_string.chars().fold(0u64, |acc, c| if c == 'X' {acc << 1 | 1} else {acc << 1});
            let v = mask_string.chars().fold(0u64, |acc, c| if c == '1' {acc << 1 | 1} else {acc << 1});
            Instruction::Mask(
                WriteMask{
                    mask: m,
                    value: v,
                }
            )
        }
        else {
            let a = l.split(|c| c == '[' || c == ']').nth(1).unwrap().parse::<u64>().unwrap();
            let v = l.split_whitespace().last().unwrap().parse::<u64>().unwrap();
            Instruction::Write(
                WriteInstruction {
                    address: a,
                    value: v,
                }
            )
        }
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        let instructions = parse_file("dat/day14_example.txt");
        println!("Instructions: {:?}", instructions);
    
        let r = execute_init_program(&instructions);
        assert_eq!(r, 165);
    }

    #[test]
    fn test_day14_part2() {
        let instructions = parse_file("dat/day14_example2.txt");
        println!("Instructions: {:?}", instructions);
    
        let r = execute_mem_decoder_init_program(&instructions);
        assert_eq!(r, 208);
    }
}