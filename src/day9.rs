use std::fs;
use itertools::Itertools;

pub fn solve() {
    let enconding = parse_encoding_file("dat/day9.txt");

    let r = check_XMAS_validity(enconding.clone(), 25).unwrap();
    println!("result: {}", r);

    let s = find_continues_sum_minmax_sum(enconding, r);
    println!("result: {}", s);
}

fn parse_encoding_file(file_name: &str) -> Vec<usize> {
    let input = fs::read_to_string(file_name).expect("file not found!");

    input.lines().map(|v| v.parse().unwrap()).collect()
}

fn check_XMAS_validity(data: Vec<usize>, preample_length: usize) -> Option::<usize> {
    // println!("vec: {:?}", data);
    let r = data.windows(preample_length+1).find(|w| {
        let (c, v) = w.split_last().unwrap();
        v.into_iter().combinations(2).all(|s| (s[0] != s[1]) && ((s[0] + s[1]) != *c))
    });
    // println!("{:?}", r.unwrap());
    return r.map(|v| *v.last().unwrap_or(&0));
}

fn find_continues_sum_minmax_sum(data: Vec<usize>, target: usize) -> usize {
    for i in 2..data.len() {
        let r = data.windows(i).find(|w| w.iter().sum::<usize>() == target);
        if r.is_some() {
            let (min_v, max_v) = r.unwrap().iter().minmax().into_option().unwrap();
            return min_v + max_v;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        let input = parse_encoding_file("dat/day9_example.txt");

        assert_eq!(check_XMAS_validity(input, 5), Some(127));
    }

    #[test]
    fn test_day9_part1_check1() {
        let input = (1..=26).collect::<Vec<usize>>();
        
        assert_eq!(check_XMAS_validity(input, 25).is_none(), true);
    }

    #[test]
    fn test_day9_part1_check2() {
        let mut input = (1..=25).collect::<Vec<usize>>();
        input.push(49);
        
        assert_eq!(check_XMAS_validity(input, 25).is_none(), true);
    }

    #[test]
    fn test_day9_part1_check3() {
        let mut input = (1..=25).collect::<Vec<usize>>();
        input.push(50);
        
        assert_eq!(check_XMAS_validity(input, 25), Some(50));
    }

    #[test]
    fn test_day9_part1_check4() {
        let mut input = (1..=25).collect::<Vec<usize>>();
        input.push(100);
        
        assert_eq!(check_XMAS_validity(input, 25), Some(100));
    }

    #[test]
    fn test_day9_part2() {
        let input = parse_encoding_file("dat/day9_example.txt");

        let r = check_XMAS_validity(input.clone(), 5).unwrap();

        let s = find_continues_sum_minmax_sum(input, r);

        assert_eq!(s, 62);
    }
}