use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let bag_rules = parse_bagrules_sheets("dat/day7.txt");
    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    bag_rules.iter().for_each(|r| add_gold_bag_rule(&mut rules, &r));
    // println!("{:?}", rules);

    let x = get_bag_names(&rules, "shiny gold");
    println!("result: {:?}", x.len());

    let x = count_bags(&rules, "shiny gold") - 1;
    println!("result: {}", x);
}

fn parse_bagrules_sheets(file_name: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_name).expect("file not found!");

    let bag_rules = input.lines()
                         .map(|l| l.split_terminator("bag")
                                   .map(|s| s.trim_start_matches(", ")
                                             .trim_start_matches("s, ")
                                             .trim_start_matches("s contain")
                                             .trim()
                                    )
                                   .filter(|l| *l != "." && *l != "s.")
                                   .map(String::from)
                                   .collect())
                         .collect::<Vec<Vec<String>>>();
    // println!("{:?}", bag_rules);

    return bag_rules;
}

fn add_gold_bag_rule(rules: &mut HashMap<String, Vec<(usize, String)>>, new_rule: &Vec<String>) {
    let r: Vec<(usize, String)> = new_rule.iter().skip(1).map(|r| (r.split_at(2).0.trim().parse::<usize>().unwrap_or(0), String::from(r.split_at(2).1.trim()))).collect();
    // println!("{:?}", r);

    rules.insert(new_rule[0].clone(), r.clone());
}

fn get_bag_names(rules: &HashMap<String, Vec<(usize, String)>>, target_bag: &str) -> HashSet<String> {
    // println!("{}", target_bag);
    let mut bags: HashSet<String> = HashSet::new();
    rules.iter().filter(|(_, v)| {
        // println!("contains: {:?}", v);
        v.iter().any(|(_, b)| {
            // println!("bag: {:?}", (x, b));
            b == target_bag
        })
    }).for_each(|(k, _)| {
        // counts duplicates entries, see bright white and muted yellow
        bags.insert(k.to_string());
        bags = bags.union(&get_bag_names(rules, k)).map(String::from).collect();
    });

    return bags;
}

fn count_bags(rules: &HashMap<String, Vec<(usize, String)>>, target_bag: &str) -> usize {
    rules.get(target_bag).unwrap_or(&vec![]).iter().map(|(s, b)| s * count_bags(rules, b)).sum::<usize>() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let bag_rules = parse_bagrules_sheets("dat/day7_example.txt");
        let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
        bag_rules.iter().for_each(|r| add_gold_bag_rule(&mut rules, &r));
    
        let x = get_bag_names(&rules, "shiny gold");

        assert_eq!(x.len(), 4);
    }

    #[test]
    fn test_day7_part2_1() {
        let bag_rules = parse_bagrules_sheets("dat/day7_example.txt");
        let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
        bag_rules.iter().for_each(|r| add_gold_bag_rule(&mut rules, &r));
    
        let x = count_bags(&rules, "shiny gold") - 1;

        assert_eq!(x, 32);
    }

    #[test]
    fn test_day7_part2_2() {
        let bag_rules = parse_bagrules_sheets("dat/day7_example2.txt");
        let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
        bag_rules.iter().for_each(|r| add_gold_bag_rule(&mut rules, &r));
    
        let x = count_bags(&rules, "shiny gold") - 1;

        assert_eq!(x, 126);
    }
}