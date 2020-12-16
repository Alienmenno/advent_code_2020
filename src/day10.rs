use std::fs;
use std::collections::HashSet;

pub fn solve() {
    let adapters = parse_adapter_file("dat/day10.txt");
    println!("{:?}", adapters);
    
    let r = sum_jolt_diffs(&adapters);
    println!("result: {}", r);

    let r = count_configs(&adapters);
    println!("result: {:?}", r);
}

fn parse_adapter_file(file_name: &str) -> Vec<usize> {
    let input = fs::read_to_string(file_name).expect("file not found!");

    let mut adapters: Vec<usize> = input.lines().map(|v| v.parse().unwrap()).collect();

    adapters.sort();
    adapters.insert(0, 0); // add wall jolts
    adapters.push(adapters.last().unwrap() + 3); // add device jolts

    return adapters;
}

fn sum_jolt_diffs(adapters: &Vec<usize>) -> usize {
    let jolt_diffs = adapters.windows(2).map(|w| w[1] - w[0]).collect::<Vec<usize>>();
    // println!("{:?}", jolt_diffs);
    let ones = jolt_diffs.iter().filter(|j| **j == 1).collect::<Vec<&usize>>().len();
    let threes = jolt_diffs.iter().filter(|j| **j == 3).collect::<Vec<&usize>>().len();
    return ones * threes;
}

#[allow(dead_code)]
fn count_possible_configs(adapters: &Vec<usize>, adapter_seq: & mut HashSet<Vec<usize>>) {
    adapters.windows(3).enumerate().filter_map(|w| {
        if (w.1[2] - w.1[0]) <= 3 {Some(w.0 + 1)}
        else {None}
    }).for_each(|i| {
        let mut new_seq = adapters.clone();
        new_seq.remove(i);

        if !adapter_seq.contains(&new_seq) {
            adapter_seq.insert(new_seq.clone());
            count_possible_configs(&new_seq, adapter_seq);
        }
    });
}

fn count_configs(adapters: &Vec<usize>) -> usize {
    let jolt_diffs = adapters.windows(2).map(|w| w[1] - w[0]).collect::<Vec<usize>>();
    let c = jolt_diffs.as_slice().split(|n| *n != 1)
                                //  .filter(|s| !s.is_empty())
                                 .map(|s| {
                                     let r = tribonacci(s.len() + 1);
                                     println!("{:?} = {}", s, r);
                                     r
                                     /*let mut r = 2usize.pow((s.len() - 1) as u32);
                                     if s.len() > 3 {
                                         r -= s.len() - 3;
                                     }
                                     r*/
                                 }).product::<usize>();
    println!("{:?}", c);
    return c;
}

fn tribonacci(stairs: usize) -> usize {
    match stairs {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 2,
        _ => tribonacci(stairs - 1) + tribonacci(stairs - 2) + tribonacci(stairs - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let adapters = parse_adapter_file("dat/day10_example.txt");
        
        let r = sum_jolt_diffs(&adapters);
        assert_eq!(r, 7 * 5);
    }

    #[test]
    fn test_day10_part1_2() {
        let adapters = parse_adapter_file("dat/day10_example2.txt");
        
        let r = sum_jolt_diffs(&adapters);
        assert_eq!(r, 22 * 10);
    }

    #[test]
    fn test_day10_part2() {
        let adapters = parse_adapter_file("dat/day10_example.txt");
        
        println!("{:?}", adapters);

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, & mut seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 8);
    }

    #[test]
    fn test_day10_part2_2() {
        let adapters = parse_adapter_file("dat/day10_example2.txt");
        
        println!("{:?}", adapters);

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, & mut seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 19208);
    }

    #[test]
    fn test_day10_part2_2seq() {
        let adapters = vec![0,1,2,5];
        // [0,1,2,5]
        // [0,2,5]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 2);
    }

    #[test]
    fn test_day10_part2_3seq() {
        let adapters = vec![0,1,2,3,6];
        // [0,1,2,3,6]
        // [0,1,3,6]
        // [0,2,3,6]
        // [0,3,6]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 4);
    }

    #[test]
    fn test_day10_part2_4seq() {
        let adapters = vec![0,1,2,3,4,7];
        // [0,1,2,3,4,7]
        // [0,1,2,4,7]
        // [0,1,3,4,7]
        // [0,1,4,7]
        // [0,2,3,4,7]
        // [0,2,4,7]
        // [0,3,4,7]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 7);
    }

    #[test]
    fn test_day10_part2_5seq() {
        let adapters = vec![0,1,2,3,4,5,8];
        // [0,1,2,3,4,5,8]
        // [0,1,2,3,5,8]
        // [0,1,2,4,5,8]
        // [0,1,2,5,8]
        // [0,1,3,4,5,8]
        // [0,1,3,5,8]
        // [0,1,4,5,8]
        // [0,2,3,4,5,8]
        // [0,2,3,5,8]
        // [0,2,4,5,8]
        // [0,2,5,8]
        // [0,3,4,5,8]
        // [0,3,5,8]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 13);
    }

    #[test]
    #[ignore]
    fn test_day10_part2_5seq2() {
        let adapters = vec![0,1,2,4,5,6,9];
        // [0,1,2,4,5,6,9]
        // [0,1,2,4,6,9]
        // [0,1,2,5,6,9]
        // [0,1,4,5,6,9]
        // [0,1,4,6,9]
        // [0,2,4,5,6,9]
        // [0,2,4,6,9]
        // [0,2,5,6,9]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 8);
    }

    #[test]
    fn test_day10_part2_5seq3() {
        let adapters = vec![0,1,2,5,6,7,10];
        // [0,1,2,5,6,7,10]
        // [0,1,2,5,7,10]
        // [0,2,5,6,7,10]
        // [0,2,5,7,10]

        let mut seq: HashSet<Vec<usize>> = HashSet::new();
        count_possible_configs(&adapters, &mut seq);
        println!("{:?}", seq);
        let r = count_configs(&adapters);
        assert_eq!(r, 4);
    }
}