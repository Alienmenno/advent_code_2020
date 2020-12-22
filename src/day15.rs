use std::fs;
use std::collections::HashMap;

pub fn solve() {
    let nums = parse_file("dat/day15.txt");
    // println!("Starting numbers: {:?}", nums);

    let r = play_number_game(&nums, 2020);
    println!("result: {:?}", r);

    let r = play_number_game(&nums, 30000000);
    println!("result: {:?}", r);
}

fn play_number_game(starting_numbers: &Vec<usize>, iterations: usize) -> usize {
    let mut nums = starting_numbers.clone();
    let mut num_map: HashMap<usize, usize> = HashMap::new();

    for i in 0..iterations-1 {
        let num = nums[i];
        if !num_map.contains_key(&num) {
            if i == (nums.len() - 1) {
                nums.push(0);
            }
        }
        else {
            let next = i - num_map[&num];
            nums.push(next);
        }
        num_map.insert(num, i);
    }

    return *nums.last().unwrap();
}

fn parse_file(file_name: &str) -> Vec<usize> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    return input.split(',').map(|s| s.parse().unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        let nums = parse_file("dat/day15_example.txt");
        // println!("Starting numbers: {:?}", nums);
    
        let r = play_number_game(&nums, 2020);
        assert_eq!(r, 436);
    }

    #[ignore]
    #[test]
    fn test_day15_part2() {
        let nums = parse_file("dat/day15_example.txt");
        // println!("Starting numbers: {:?}", nums);
    
        let r = play_number_game(&nums, 30000000);
        assert_eq!(r, 175594);
    }
}