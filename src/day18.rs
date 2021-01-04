use std::fs;
use std::str::Chars;

pub fn solve() {
    let problems = parse_file("dat/day18.txt");
    // println!("input: {:?}", problems);

    let problem_sum = problems.iter().map(|p| p.solve_left()).sum::<u64>();
    println!("Result: {}", problem_sum);

    let problem_sum = problems.iter().map(|p| p.solve_addition()).sum::<u64>();
    println!("Result: {}", problem_sum);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    ADD,
    MUL,
    VAL(u64),
    GROUP(Problem),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    equation: Vec<Op>,
}

impl Problem {
    fn solve_left(&self) -> u64 {
        let mut cur_op = Op::ADD;
        self.equation.iter().fold(0, |acc, op| {
            match op {
                Op::ADD => {cur_op = Op::ADD; acc},
                Op::MUL => {cur_op = Op::MUL; acc},
                Op::VAL(x) => match cur_op {
                    Op::ADD => acc + x,
                    Op::MUL => acc * x,
                     _  => panic!(),
                },
                Op::GROUP(p) => match cur_op {
                    Op::ADD => acc + p.solve_left(),
                    Op::MUL => acc * p.solve_left(),
                     _  => panic!(),
                },
            }
        })
    }

    fn solve_addition(&self) -> u64 {
        self.equation
            .windows(3)
            .map(|x| if x[1] == Op::ADD {
                    Op::VAL(x[0] + x[2])
                }
                else {
                    
                }
            )
            .for_each(|x| println!("{:?}", x));
        return 0;
    }
}

fn parse_file(file_name: &str) -> Vec<Problem> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    input.lines().map(|l| parse_problem(l)).collect()
}

fn parse_problem(problem_str: &str) -> Problem {
    let stripped_str = problem_str.replace(" ", "");
    let mut it = stripped_str.chars();
    
    parse_tokens(&mut it)
}

fn parse_tokens(it: &mut Chars) -> Problem {
    let mut problem: Vec<Op> = Vec::new();

    while let Some(c) = it.next() {
        match c {
            '+' => problem.push(Op::ADD),
            '*' => problem.push(Op::MUL),
            '(' => problem.push(Op::GROUP(parse_tokens(&mut *it))),
            ')' => {
                return Problem {
                    equation: problem,
                }
            },
             x  => problem.push(Op::VAL(x.to_string().parse::<u64>().unwrap())),
        }
    }

    Problem {
        equation: problem,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(input, expected,
        case("1 + 2 * 3 + 4 * 5 + 6", 71),
        case("1 + (2 * 3) + (4 * (5 + 6))", 51),
        case("2 * 3 + (4 * 5)", 26),
        case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    )]
    fn test_day18_part1(input: &str, expected: u64) {
        let problem = parse_problem(input);
        println!("{:?}", problem);

        let r = problem.solve_left();
        assert_eq!(r, expected);
    }

    #[rstest(input, expected,
        case("1 + 2 * 3 + 4 * 5 + 6", 231),
        // case("1 + (2 * 3) + (4 * (5 + 6))", 51),
        // case("2 * 3 + (4 * 5)", 46),
        // case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        // case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        // case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    )]
    fn test_day18_part2(input: &str, expected: u64) {
        let problem = parse_problem(input);
        println!("{:?}", problem);

        let r = problem.solve_addition();
        assert_eq!(r, expected);
    }
}