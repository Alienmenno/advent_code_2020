use std::fs;

pub fn solve() {
    let slope = parse_slope("dat/day3.txt");
    // println!("{:?}", slope);
    
    let descend_profile = (3, 1);
    let trees_hit = count_trees_on_slope_descend(&slope, &descend_profile);
    println!("result: {}", trees_hit);

    let descend_profiles_to_check = vec![(1,1), (3,1), (5,1), (7,1), (1, 2)];
    let product_for_profiles = check_descend_profiles(&slope, &descend_profiles_to_check);
    println!("result: {}", product_for_profiles)
}

fn parse_slope(file_name: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    // println!("{:?}", input);

    let slope: Vec<Vec<char>> = input.lines().map(|s| s.trim().chars().collect()).collect();

    return slope;
}

fn count_trees_on_slope_descend(slope: &Vec<Vec<char>>, descend_profile: &(usize, usize)) -> usize {
    let mut w = 0;
    let mut h = 0;

    let mut trees_hit = 0;
    while h < slope.len() {
        if slope[h][w] == '#' {
            trees_hit += 1
        }

        w = (w + descend_profile.0) % slope[h].len();
        h += descend_profile.1;
        // println!("{}, {}", w, h);
    }

    return trees_hit;
}

fn check_descend_profiles(slope: &Vec<Vec<char>>, profiles: &Vec<(usize, usize)>) -> usize {
    let product_profiles = profiles.iter().map(|p| count_trees_on_slope_descend(&slope, p)).into_iter().product();

    return product_profiles;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let slope = parse_slope("dat/day3_example.txt");
        let descend_profile = (3, 1);
    
        let trees_hit = count_trees_on_slope_descend(&slope, &descend_profile);

        assert_eq!(trees_hit, 7);
    }

    #[test]
    fn test_day3_part2() {
        let slope = parse_slope("dat/day3_example.txt");
        let descend_profiles_to_check = vec![(1,1), (3,1), (5,1), (7,1), (1, 2)];
        let product_for_profiles = check_descend_profiles(&slope, &descend_profiles_to_check);

        assert_eq!(product_for_profiles, 336);
    }
}