use std::fs;
use std::cmp;
use std::fmt;
use itertools::Itertools;

pub fn solve() {
    let seats = parse_seat_file("dat/day11.txt");
    // println!("{:?}", seats);

    let mut seating_arrangement1 = PlaneSeating::create_arrangement(&seats);
    println!("Pre:\n{}", seating_arrangement1);
    while seating_arrangement1.tick() {}
    println!("Post:\n{}", seating_arrangement1);

    let s = seating_arrangement1.count_seats_for_type(PlaneSeat::TakenSeat);
    println!("result: {}", s);

    let mut seating_arrangement2 = PlaneSeating::create_arrangement(&seats);
    println!("Pre:\n{}", seating_arrangement2);
    while seating_arrangement2.tick2() {
        // println!("{}", seating_arrangement2);
    }
    println!("Post:\n{}", seating_arrangement2);

    let s = seating_arrangement2.count_seats_for_type(PlaneSeat::TakenSeat);
    println!("result: {}", s);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PlaneSeat {
    EmptySeat,
    TakenSeat,
    Floor,
}

#[derive(Debug, PartialEq, Eq)]
struct PlaneSeating {
    width: usize,
    height: usize,
    aisles: Vec<PlaneSeat>,
}

impl PlaneSeating {
    fn create_arrangement(seating: &Vec<String>) -> PlaneSeating {
        let width = seating.first().unwrap().len();
        let height = seating.len();

        let aisles = seating.iter().flat_map(|s| s.chars().map(|c| match c {
            'L' => PlaneSeat::EmptySeat,
            '#' => PlaneSeat::TakenSeat,
            '.' => PlaneSeat::Floor,
             _  => PlaneSeat::Floor
        })).collect();

        PlaneSeating {width, height, aisles,}
    }

    fn get_index(&self, r: i64, c: i64) -> Option<PlaneSeat> {
        if r >= 0 && r <= (self.height as i64 - 1)
        && c >= 0 && c <= (self.width as i64 - 1) {
            Some(self.index(r as usize, c as usize))
        }
        else {
            None
        }
    }

    fn index(&self, row: usize, column: usize) -> PlaneSeat {
        self.aisles[(row * self.width + column)]
    }

    fn neighbor_seat_count(&self, row: usize, column: usize, seat_type: PlaneSeat) -> usize {
        let row_ids =    vec![row.saturating_sub(1),       row, cmp::min(row + 1,    self.height - 1)];
        let column_ids = vec![column.saturating_sub(1), column, cmp::min(column + 1, self.width - 1)];

        row_ids.into_iter().cartesian_product(column_ids).unique().filter(|&(r, c)| {
            (r != row || c != column) && self.index(r, c) == seat_type
        }).count()
    }

    fn visible_seat_count(&self, row: usize, column: usize, seat_type: PlaneSeat) -> usize {
        let line_of_sight: Vec<(i64, i64)> = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        line_of_sight.iter().filter_map(|&(r, c)| {
            let mut i = (row as i64 + r, column as i64 + c);
            let mut s = self.get_index(i.0, i.1);
            while s == Some(PlaneSeat::Floor) {
                i = (i.0 as i64 + r, i.1 as i64 + c);
                s = self.get_index(i.0, i.1);
            }
            s
        }).filter(|&s| s == seat_type).count()
    }

    fn count_seats_for_type(&self, seat_type: PlaneSeat) -> usize {
        self.aisles.iter().filter(|&s| *s == seat_type).count()
    }

    fn tick(&mut self) -> bool {
        let mut seating_changed = false;
        self.aisles = (0..self.height).cartesian_product(0..self.width).map(|(r, c)| {
            let seat = self.index(r, c);
            let neighbor_count = self.neighbor_seat_count(r, c, PlaneSeat::TakenSeat);
            match (seat, neighbor_count) {
                (PlaneSeat::EmptySeat, 0) => {seating_changed = true; PlaneSeat::TakenSeat},
                (PlaneSeat::TakenSeat, x) if x >= 4 => {seating_changed = true; PlaneSeat::EmptySeat},
                (seat, _) => seat,
            }
        }).collect();

        return seating_changed;
    }

    fn tick2(&mut self) -> bool {
        let mut seating_changed = false;
        self.aisles = (0..self.height).cartesian_product(0..self.width).map(|(r, c)| {
            let seat = self.index(r, c);
            let neighbor_count = self.visible_seat_count(r, c, PlaneSeat::TakenSeat);
            match (seat, neighbor_count) {
                (PlaneSeat::EmptySeat, 0) => {seating_changed = true; PlaneSeat::TakenSeat},
                (PlaneSeat::TakenSeat, x) if x >= 5 => {seating_changed = true; PlaneSeat::EmptySeat},
                (seat, _) => seat,
            }
        }).collect();

        return seating_changed;
    }
}

impl fmt::Display for PlaneSeating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width: {}, height: {}, total length: {}\n",
                  self.width, self.height, self.height * self.width)?;
        for line in self.aisles.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    PlaneSeat::EmptySeat => '◻',
                    PlaneSeat::TakenSeat => '◼',
                    PlaneSeat::Floor     => ' ',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn parse_seat_file(file_name: &str) -> Vec<String> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    input.lines().map(String::from).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1() {
        let seats = parse_seat_file("dat/day11_example.txt");
    
        let mut seating_arrangement = PlaneSeating::create_arrangement(&seats);
        println!("Pre:\n{}", seating_arrangement);
        while seating_arrangement.tick(){}
        println!("Post:\n{}", seating_arrangement);
    
        let s = seating_arrangement.count_seats_for_type(PlaneSeat::TakenSeat);
        assert_eq!(s, 37);
    }

    #[test]
    fn test_day11_part2() {
        let seats = parse_seat_file("dat/day11_example.txt");
    
        let mut seating_arrangement = PlaneSeating::create_arrangement(&seats);
        println!("Pre:\n{}", seating_arrangement);
        while seating_arrangement.tick2(){}
        println!("Post:\n{}", seating_arrangement);
    
        let s = seating_arrangement.count_seats_for_type(PlaneSeat::TakenSeat);
        assert_eq!(s, 26);
    }

    #[test]
    fn test_day11_part2_vis_seats_1() {
        let seats = parse_seat_file("dat/day11_example_part2_1.txt");
        let seating_arrangement = PlaneSeating::create_arrangement(&seats);

        let vis_seats = seating_arrangement.visible_seat_count(4, 3, PlaneSeat::TakenSeat);
        assert_eq!(vis_seats, 8);
    }

    #[test]
    fn test_day11_part2_vis_seats_2() {
        let seats = parse_seat_file("dat/day11_example_part2_2.txt");
        let seating_arrangement = PlaneSeating::create_arrangement(&seats);

        let vis_seats = seating_arrangement.visible_seat_count(1, 1, PlaneSeat::TakenSeat);
        assert_eq!(vis_seats, 0);
    }

    #[test]
    fn test_day11_part2_vis_seats_2_2() {
        let seats = parse_seat_file("dat/day11_example_part2_2.txt");
        let seating_arrangement = PlaneSeating::create_arrangement(&seats);

        let vis_seats = seating_arrangement.visible_seat_count(1, 3, PlaneSeat::TakenSeat);
        assert_eq!(vis_seats, 1);
    }

    #[test]
    fn test_day11_part2_vis_seats_3() {
        let seats = parse_seat_file("dat/day11_example_part2_3.txt");
        let seating_arrangement = PlaneSeating::create_arrangement(&seats);

        let vis_seats = seating_arrangement.visible_seat_count(3, 3, PlaneSeat::TakenSeat);
        assert_eq!(vis_seats, 0);
    }

    #[test]
    fn test_day11_part2_vis_seats_4() {
        let seats = parse_seat_file("dat/day11_example_part2_3.txt");
        let seating_arrangement = PlaneSeating::create_arrangement(&seats);

        let vis_seats = seating_arrangement.visible_seat_count(0, 0, PlaneSeat::TakenSeat);
        assert_eq!(vis_seats, 2);
    }
}