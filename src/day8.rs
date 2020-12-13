use std::fs;
use std::collections::HashSet;

pub fn solve() {
    let opcodes = parse_opcodes("dat/day8.txt");

    let (_, acc) = find_execute_loop(&opcodes);
    println!("result: {:?}", acc);

    let acc_fix = fix_opcodes(&opcodes);
    println!("result: {}", acc_fix);
}

fn parse_opcodes(file_name: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    let opcodes = input.lines()
                       .map(|l| l.split_whitespace().map(String::from).collect())
                       .collect::<Vec<Vec<String>>>();
    // println!("{:?}", opcodes);
    return opcodes;
}

fn find_execute_loop(opcodes: &Vec<Vec<String>>) -> (bool, i32) {
    let mut st_ptr: usize = 0;
    let mut acc: i32 = 0;
    let mut executed_codes: HashSet<usize> = HashSet::new();

    while !executed_codes.contains(&st_ptr) && st_ptr < opcodes.len() {
        executed_codes.insert(st_ptr);
        // println!("opcode: {:?}, executed: {:?}, st_ptr: {}, acc: {}", opcodes[st_ptr], executed_codes, st_ptr, acc);
        match opcodes[st_ptr][0].as_str() {
            "jmp" => st_ptr = ((st_ptr as i64) + opcodes[st_ptr][1].parse::<i64>().unwrap_or(0)) as usize,
            "acc" => {acc += opcodes[st_ptr][1].parse().unwrap_or(0); st_ptr += 1},
            "nop" => st_ptr += 1,
            _ => println!("Error")
        };
        // println!("st_ptr: {}, acc: {}", st_ptr, acc);
    }

    let endless = !executed_codes.contains(&(opcodes.len() - 1));
    return (endless, acc);
}

fn fix_opcodes(opcodes: &Vec<Vec<String>>) -> i32 {
    let mut r = (true, 0);
    let mut i = 0;

    while r.0 {
        // println!("i: {}", i);
        let mut op = opcodes.clone();
        match op[i][0].as_str() {
            "jmp" => op[i][0] = "nop".to_string(),
            "nop" => op[i][0] = "jmp".to_string(),
            _ => {i += 1; continue}
        }
        // println!("op: {:?}", op);
        r = find_execute_loop(&op);
        // println!("r: {:?}", r);
        i += 1;
    }
    
    return r.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let opcodes = parse_opcodes("dat/day8_example.txt");

        let (_, acc) = find_execute_loop(&opcodes);

        assert_eq!(acc, 5);
    }

    #[test]
    fn test_day8_part2() {
        let opcodes = parse_opcodes("dat/day8_example.txt");

        let acc = fix_opcodes(&opcodes);

        assert_eq!(acc, 8);
    }
}