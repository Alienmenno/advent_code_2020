use std::fs;
use std::collections::HashSet;

pub fn solve() {
    let group_answers = parse_answer_sheets("dat/day6.txt");

    let group_count = count_group_answers(&group_answers);
    println!("result: {}", group_count);

    let common_group_count = count_common_group_answers(&group_answers);
    println!("result: {}", common_group_count);
}

fn parse_answer_sheets(file_name: &str) -> Vec<String> {
    let input = fs::read_to_string(file_name).expect("file not found!");

    let group_answers = input.lines()
                             .map(String::from)
                             .collect::<Vec<String>>()
                             .split(|l| l.is_empty())
                             .map(|ls| ls.join(" ")).collect();
    // println!("{:?}", group_answers);

    return group_answers;
}

fn count_group_answers(group_answers: &Vec<String>) -> usize {
    group_answers.iter().map(|g| {
            let answer_set: HashSet<char> = g.split_whitespace().collect::<Vec<&str>>().join("").chars().collect();
            answer_set.len()
        }).sum()
}

fn count_common_group_answers(group_answers: &Vec<String>) -> usize {
    let question_ids: HashSet<char> = ('a'..='z').into_iter().collect();
    // println!("{:?}", question_ids);
    let count = group_answers.iter().map(|g| g.split_whitespace()
                                              .collect::<Vec<&str>>()
                                              .iter()
                                              .map(|a| a.chars().collect::<HashSet<char>>())
                                              .fold(question_ids.clone(), |q, a| q.intersection(&a)
                                                                                  .cloned()
                                                                                  .collect())
                                              .len()
                                        ).sum();
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let group_answers = parse_answer_sheets("dat/day6_example.txt");

        let group_count = count_group_answers(&group_answers);
        println!("{:?}", group_count);

        assert_eq!(group_count, 11);
    }

    #[test]
    fn test_day6_part2() {
        let group_answers = parse_answer_sheets("dat/day6_example.txt");

        let group_count = count_common_group_answers(&group_answers);
        println!("{:?}", group_count);

        assert_eq!(group_count, 6);
    }
}