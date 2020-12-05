use std::fs;

pub fn solve() {
    let password_data = parse_password_input("dat/day2.txt");
    // println!("password_data: {:?}", password_data);

    let nr_valid_passwords = find_valid_passwords1(&password_data);
    println!("result: {}", nr_valid_passwords);

    let nr_valid_passwords = find_valid_passwords2(&password_data);
    println!("result: {}", nr_valid_passwords);
}

fn parse_password_input(file_name: &str) -> Vec<(usize, usize, char, String)> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    
    let password_data = input.lines().map(|s| {
        let tmp: Vec<&str> = s.split(&[' ','-'][..]).collect();
        (
            tmp[0].parse::<usize>().unwrap(),
            tmp[1].parse::<usize>().unwrap(),
            tmp[2].chars().next().unwrap(),
            tmp[3].parse::<String>().unwrap()
        )
    }).collect();
    
    return password_data;
}

fn find_valid_passwords1(passwords: &Vec<(usize, usize, char, String)>) -> usize {
    let mut valid_count = 0;

    for pass in passwords.iter() {
        let char_count = pass.3.chars().filter(|c| *c == pass.2).count();

        if char_count >= pass.0 && char_count <= pass.1 {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn find_valid_passwords2(passwords: &Vec<(usize, usize, char, String)>) -> usize {
    let valid_count = passwords.iter()
        .filter(|p| {
            let f = p.3.chars().nth(p.0 - 1).unwrap_or('0') == p.2;
            let s = p.3.chars().nth(p.1 - 1).unwrap_or('0') == p.2;
            !f != !s
        })
        .count();

    return valid_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let password_data = parse_password_input("dat/day2_example.txt");

        let nr_valid_passwords = find_valid_passwords1(&password_data);

        assert_eq!(nr_valid_passwords, 2);
    }

    #[test]
    fn test_day2_part2() {
        let password_data = parse_password_input("dat/day2_example.txt");
        let nr_valid_passwords = find_valid_passwords2(&password_data);

        assert_eq!(nr_valid_passwords, 1);
    }
}