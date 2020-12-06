use std::fs;

pub fn solve() {
    let boarding_passes = det_board_passes("dat/day5.txt");

    let mut seat_ids: Vec<usize> = boarding_passes.iter().map(|p| det_seat_id(p)).collect();
    seat_ids.sort_unstable();
    // println!("{:?}", seat_ids);

    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("result: {}", max_seat_id);

    let my_seat = find_missing_seat(&seat_ids);
    println!("result: {:?}", my_seat);
}

fn find_missing_seat(seat_ids: &Vec<usize>) -> usize {
    seat_ids.windows(2).find(|sp| (sp[0] + 1) != sp[1]).unwrap()[0] + 1
}

fn det_board_passes(file_name: &str) -> Vec<String> {
    let input = fs::read_to_string(file_name).expect("file not found!");

    let boarding_passes: Vec<String> = input.lines().map(String::from).collect();
    // println!("{:?}", boarding_passes);
    return boarding_passes;
}

fn det_seat_id(boarding_pass: &str) -> usize {
    let (row_code, column_code) = boarding_pass.split_at(7);
    let seat_row = det_seat_axis(row_code, (0, 127));
    let seat_column = det_seat_axis(column_code, (0, 7));
    // println!("{},{}", seat_row, seat_column);
    return seat_row * 8 + seat_column;
}

fn det_seat_axis(boarding_pass: &str, axis: (usize, usize)) -> usize {
    // println!("{:?}", axis);
    let mid_point = ((axis.1 + 1) - axis.0) / 2 + axis.0;
    match  boarding_pass.split_at(1) {
        ("F", "")|("L", "") => return axis.0,
        ("B", "")|("R", "") => return axis.1,
        ("F", re)|("L", re) => return det_seat_axis(re, (axis.0, mid_point-1)),
        ("B", re)|("R", re) => return det_seat_axis(re, (mid_point, axis.1)),
        _ => return 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        let boarding_passes = det_board_passes("dat/day5_example.txt");

        let seat_ids: Vec<usize> = boarding_passes.iter().map(|p| det_seat_id(p)).collect();
        println!("{:?}", seat_ids);

        assert_eq!(seat_ids, vec![357, 567, 119, 820]);
    }

    #[test]
    fn test_day5_edge_test() {
        let boarding_passes = det_board_passes("dat/day5_edge.txt");

        let seat_ids: Vec<usize> = boarding_passes.iter().map(|p| det_seat_id(p)).collect();
        println!("{:?}", seat_ids);

        assert_eq!(seat_ids, vec![0, 7, 1016, 1023, 507, 1014, 515, 14]);
    }
}