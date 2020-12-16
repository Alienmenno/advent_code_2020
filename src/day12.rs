use std::fs;

pub fn solve() {
    let actions = parse_ship_instruction_file("dat/day12.txt");
    // println!("{:?}", actions);

    let mut ship = Ship {
        ship_location: Coordinate {x: 0, y: 0,},
        waypoint_location: Coordinate {x: 0, y: 0,},
        direction: Direction::East,
    };
    ship.plot_coarse(&actions);
    let r = ship.distance_traveled();
    println!("result: {}", r);

    // Reset ship
    ship = Ship {
        ship_location: Coordinate {x: 0, y: 0,},
        waypoint_location: Coordinate {x: -10, y: 1,},
        direction: Direction::East,
    };
    ship.plot_waypoint_coarse(&actions);
    let r = ship.distance_traveled();
    println!("result: {}", r);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ShipAction {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    MoveForward,
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West, 
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Ship {
    ship_location: Coordinate,
    direction: Direction,
    waypoint_location: Coordinate,    
}

impl Ship {
    fn plot_coarse(&mut self, actions: &Vec<(ShipAction, i64)>) {
        actions.into_iter().for_each(|&action| {
            match action {
                (ShipAction::RotateLeft,  v) => self.rotate_ship(-v),
                (ShipAction::RotateRight, v) => self.rotate_ship( v),
                (ShipAction::MoveForward, v) => Ship::move_coordinate(&mut self.ship_location, self.direction, v),
                (ShipAction::MoveNorth,   v) => Ship::move_coordinate(&mut self.ship_location, Direction::North, v),
                (ShipAction::MoveSouth,   v) => Ship::move_coordinate(&mut self.ship_location, Direction::South, v),
                (ShipAction::MoveEast,    v) => Ship::move_coordinate(&mut self.ship_location, Direction::East, v),
                (ShipAction::MoveWest,    v) => Ship::move_coordinate(&mut self.ship_location, Direction::West, v),
            }
        });
    }

    fn rotate_ship(&mut self, rotation: i64) {
        static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East,
                                             Direction::South, Direction::West,];
        let r_index = Ship::rotation_to_cardinal_index(rotation);
        self.direction = *DIRECTIONS.iter()
                                    .cycle()
                                    .skip_while(|&&d| d != self.direction)
                                    .nth(r_index).unwrap();
    }

    //--------------------------------------------------------------------------
    fn plot_waypoint_coarse(&mut self, actions: &Vec<(ShipAction, i64)>) {
        actions.into_iter().for_each(|&action| {
            match action {
                (ShipAction::RotateLeft,  v) => self.rotate_waypoint(-v),
                (ShipAction::RotateRight, v) => self.rotate_waypoint( v),
                (ShipAction::MoveForward, v) => self.move_by_waypoint(v),
                (ShipAction::MoveNorth,   v) => Ship::move_coordinate(&mut self.waypoint_location, Direction::North, v),
                (ShipAction::MoveSouth,   v) => Ship::move_coordinate(&mut self.waypoint_location, Direction::South, v),
                (ShipAction::MoveEast,    v) => Ship::move_coordinate(&mut self.waypoint_location, Direction::East, v),
                (ShipAction::MoveWest,    v) => Ship::move_coordinate(&mut self.waypoint_location, Direction::West, v),
            }
        });
    }

    fn move_by_waypoint(&mut self, distance: i64) {
        self.ship_location.x += self.waypoint_location.x * distance;
        self.ship_location.y += self.waypoint_location.y * distance;
    }

    fn rotate_waypoint(&mut self, rotation: i64) {
        let r_index = Ship::rotation_to_cardinal_index(rotation);

        self.waypoint_location = match r_index {
            1 => Coordinate{x: -1*self.waypoint_location.y, y:    self.waypoint_location.x}, //  90 degrees
            2 => Coordinate{x: -1*self.waypoint_location.x, y: -1*self.waypoint_location.y}, // 180 degrees
            3 => Coordinate{x:    self.waypoint_location.y, y: -1*self.waypoint_location.x}, // 270 degrees
            _ => Coordinate{x: 0, y: 0}, // error
        };
    }

    //--------------------------------------------------------------------------
    fn rotation_to_cardinal_index(r: i64) -> usize {
        (((r / 90) + 4) % 4) as usize
    }

    fn move_coordinate(coord: &mut Coordinate, direction: Direction, distance: i64) {
        match direction {
            Direction::North => coord.y += distance,
            Direction::South => coord.y -= distance,
            Direction::East  => coord.x -= distance,
            Direction::West  => coord.x += distance,
        }
    }

    fn distance_traveled(&self) -> i64 {
        self.ship_location.x.abs() + self.ship_location.y.abs()
    }
}

fn parse_ship_instruction_file(file_name: &str) -> Vec<(ShipAction, i64)> {
    let input = fs::read_to_string(file_name).expect("file not found!");
    input.lines().map(|l| {
        let (instr, value) = l.split_at(1);
        (match instr.parse().unwrap() {
            'N' => ShipAction::MoveNorth,
            'S' => ShipAction::MoveSouth,
            'E' => ShipAction::MoveEast,
            'W' => ShipAction::MoveWest,
            'F' => ShipAction::MoveForward,
            'L' => ShipAction::RotateLeft,
            'R' => ShipAction::RotateRight,
            _ => ShipAction::MoveForward,
        }, value.parse().unwrap())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let actions = parse_ship_instruction_file("dat/day12_example.txt");
        println!("{:?}", actions);
    
        let mut ship = Ship {
            ship_location: Coordinate {x: 0, y: 0,},
            waypoint_location: Coordinate {x: 0, y: 0,},
            direction: Direction::East,
        };
        ship.plot_coarse(&actions);
        let r = ship.distance_traveled();
        assert_eq!(r, 25)
    }

    #[test]
    fn test_day12_part1_2() {
        let actions = parse_ship_instruction_file("dat/day12_example2.txt");
        println!("{:?}", actions);
    
        let mut ship = Ship {
            ship_location: Coordinate {x: 0, y: 0,},
            waypoint_location: Coordinate {x: 0, y: 0,},
            direction: Direction::East,
        };
        ship.plot_coarse(&actions);
        let r = ship.distance_traveled();
        assert_eq!(r, 35)
    }

    #[test]
    fn test_day12_part2() {
        let actions = parse_ship_instruction_file("dat/day12_example.txt");
        println!("{:?}", actions);
    
        let mut ship = Ship {
            ship_location: Coordinate {x: 0, y: 0,},
            waypoint_location: Coordinate {x: -10, y: 1,},
            direction: Direction::East,
        };
        ship.plot_waypoint_coarse(&actions);
        let r = ship.distance_traveled();
        assert_eq!(r, 286)
    }

    #[test]
    fn test_day12_part2_2() {
        let actions = parse_ship_instruction_file("dat/day12_example2.txt");
        println!("{:?}", actions);
    
        let mut ship = Ship {
            ship_location: Coordinate {x: 0, y: 0,},
            waypoint_location: Coordinate {x: -10, y: 1,},
            direction: Direction::East,
        };
        ship.plot_waypoint_coarse(&actions);
        let r = ship.distance_traveled();
        assert_eq!(r, 346)
    }
}