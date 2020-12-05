use std::fs;

pub fn solve() {
    let numbers = parse_int_txt_file("dat/day1_part1.txt");

    let r = find_pairs_to_sum_to_target(&numbers, 2020);
    println!("result: {}", r);

    let r = find_triplets_to_sum_to_target(&numbers, 2020);

    println!("result: {}", r);
}

fn parse_int_txt_file(file_name: &str) -> Vec<i32> {
    let contents = fs::read_to_string(file_name).unwrap();
    let str_numbers = contents.lines().collect::<Vec<&str>>();
    let numbers = str_numbers.into_iter().map(|s| s.parse()).collect::<Result<Vec<i32>, std::num::ParseIntError>>().unwrap_or_default();

    return numbers;
}

fn find_pairs_to_sum_to_target(inp_vec: &Vec<i32>, target: i32) -> i32 {
    for n in inp_vec.iter() {
        for y in inp_vec.iter() {
            if (n + y) == target {
                return n * y;
            }
        }
    }

    return 0;
}

fn find_triplets_to_sum_to_target(inp_vec: &Vec<i32>, target: i32) -> i32 {
    for x in inp_vec.iter() {
        for y in inp_vec.iter() {
            for z in inp_vec.iter() {
                if (x + y + z) == target {
                    return x * y * z;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let numbers = parse_int_txt_file("dat/day1_part1_example.txt");
        
        let r = find_pairs_to_sum_to_target(&numbers, 2020);
        
        assert_eq!(r, 514579);
    }
    
    #[test]
    fn test_day1_part2() {
        let numbers = parse_int_txt_file("dat/day1_part1_example.txt");

        let r = find_triplets_to_sum_to_target(&numbers, 2020);

        assert_eq!(r, 241861950);
    }
}
