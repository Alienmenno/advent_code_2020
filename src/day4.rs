use std::fs;
use std::collections::HashSet;

pub fn solve() {
    let passport_data = parse_passport_data("dat/day4.txt");
    // println!("{:?}", passport_data);

    let valid_passport_count = count_valid_passports1(&passport_data);
    println!("result: {}", valid_passport_count);

    let valid_passport_count = count_valid_passports2(&passport_data);
    println!("result: {}", valid_passport_count);
}

fn parse_passport_data(file_name: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    // println!("{:?}", input);

    let passport_data: Vec<Vec<String>> = input.split("\r\n\r\n")
            .map(|s| s.split_whitespace().map(String::from).collect())
            .collect();
    // println!("{:?}", passport_data);

    return passport_data;
}

fn count_valid_passports1(passport_data: &Vec<Vec<String>>) -> usize {
    let required_passport_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect(); // cid is allowed to be missing
    // println!("{:?}", required_passport_fields);
    let valid_passports = passport_data.iter()
            .filter(|p| p.iter()
                         .map(|e| e.split(':').collect::<Vec<&str>>()[0])
                         .collect::<HashSet<_>>()
                         .is_superset(&required_passport_fields))
            .count();

    return valid_passports;
}

fn count_valid_passports2(passport_data: &Vec<Vec<String>>) -> usize {
    let required_passport_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect(); // cid is allowed to be missing
    let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let valid_passports = passport_data.iter()
            .filter(|p| p.iter().all(|f| match split_to_tuple(f) {
                    ("byr", s) => {let v = s.parse::<u32>().unwrap(); v >= 1920 && v <= 2002},
                    ("iyr", s) => {let v = s.parse::<u32>().unwrap(); v >= 2010 && v <= 2020},
                    ("eyr", s) => {let v = s.parse::<u32>().unwrap(); v >= 2020 && v <= 2030},
                    ("hgt", s) => {check_height(s)},
                    ("hcl", s) => {s.starts_with('#') && s.trim_start_matches('#').chars().all(|c| c.is_ascii_hexdigit())},
                    ("ecl", s) => {eye_colors.iter().any(|&e| e == s)},
                    ("pid", s) => {s.chars().all(|c| c.is_numeric() && s.len() == 9)},
                    ("cid", _) => true,
                    _ => false,
                }) && p.iter()
                    .map(|e| e.split(':').collect::<Vec<&str>>()[0])
                    .collect::<HashSet<_>>()
                    .is_superset(&required_passport_fields)
                )
            .count();

    return valid_passports;
}

fn split_to_tuple(s: &str) -> (&str, &str) {
    let v: Vec<&str> = s.split(':').collect();
    return (v[0], v[1]);
}

fn check_height(s: &str) -> bool {
    if s.ends_with("cm") {
        let v = s.trim_end_matches("cm").parse::<u32>().unwrap();
        return v >= 150 && v <= 193;
    }
    else if s.ends_with("in") {
        let v = s.trim_end_matches("in").parse::<u32>().unwrap();
        return v >= 59 && v <= 76;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {
        let passport_data = parse_passport_data("dat/day4_example.txt");
    
        let valid_passport_count = count_valid_passports1(&passport_data);

        assert_eq!(valid_passport_count, 2);
    }

    #[test]
    fn test_day4_part2() {
        let passport_data = parse_passport_data("dat/day4_example.txt");
    
        let valid_passport_count = count_valid_passports2(&passport_data);

        assert_eq!(valid_passport_count, 2);
    }

    #[test]
    fn test_day4_part2_invalid() {
        let passport_data = parse_passport_data("dat/day4_invalid.txt");

        let valid_passport_count = count_valid_passports2(&passport_data);

        assert_eq!(valid_passport_count, 0);
    }

    #[test]
    fn test_day4_part2_valid() {
        let passport_data = parse_passport_data("dat/day4_valid.txt");

        let valid_passport_count = count_valid_passports2(&passport_data);

        assert_eq!(valid_passport_count, 4);
    }
}