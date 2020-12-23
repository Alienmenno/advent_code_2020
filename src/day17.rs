use std::fs;
use std::fmt;
use std::collections::HashSet;

pub fn solve() {
    let mut pocket_dimension = parse_file("dat/day17.txt");
    println!("Initial state: {}", pocket_dimension);
    
    pocket_dimension = pocket_dimension.skip(5).nth(0).unwrap();
    println!("Final state: {}", pocket_dimension);
    println!("iter 6: {}", pocket_dimension.activate_cubes.len());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

#[derive(Debug, Clone,)]
struct PocketDimension {
    activate_cubes: HashSet<(isize, isize, isize)>,
    min_max_x: (isize, isize),
    min_max_y: (isize, isize),
    min_max_z: (isize, isize),
}

impl PocketDimension {
    fn index(&self, x: isize, y: isize, z: isize) -> CubeState {
        if self.activate_cubes.contains(&(x, y, z)) {CubeState::Active}
        else {CubeState::Inactive}
    }

    fn neighbor_count(&self, x: isize, y: isize, z:isize, state:CubeState) -> usize {
        let neighbors: Vec<(isize, isize, isize)> = vec![
            // z=-1
            (-1, -1, -1), (0, -1, -1), (1, -1, -1),
            (-1,  0, -1), (0,  0, -1), (1,  0, -1),
            (-1,  1, -1), (0,  1, -1), (1,  1, -1),
            // z=0
            (-1, -1,  0), (0, -1,  0), (1, -1,  0),
            (-1,  0,  0),              (1,  0,  0),
            (-1,  1,  0), (0,  1,  0), (1,  1,  0),
            // z=1
            (-1, -1,  1), (0, -1,  1), (1, -1,  1),
            (-1,  0,  1), (0,  0,  1), (1,  0,  1),
            (-1,  1,  1), (0,  1,  1), (1,  1,  1),
        ];

        neighbors.iter()
                 .filter(|(xu, yu, zu)| self.index(x+xu, y+yu, z+zu) == state)
                //  .inspect(|x| println!("{:?}", x))
                 .count()
    }

    fn tick(&mut self) {
        let mut new_state: HashSet<(isize, isize, isize)> = HashSet::new();
        let range_x = (self.min_max_x.0 - 1)..(self.min_max_x.1 + 1);
        let range_y = (self.min_max_y.0 - 1)..(self.min_max_y.1 + 1);
        let range_z = (self.min_max_z.0 - 1)..(self.min_max_z.1 + 1);

        for (x, y, z) in iproduct!(range_x, range_y, range_z) {
            let ct = self.index(x, y, z);
            let anc = self.neighbor_count(x, y, z, CubeState::Active);
            let new_cube_state = match (ct, anc) {
                (CubeState::Active, x) if x == 2 || x == 3 => CubeState::Active,
                (CubeState::Inactive, 3) => CubeState::Active,
                _ => CubeState::Inactive,
            };
            // println!("Checking: {:?} -> {} has {} -> {}", (x, y, z), ct, anc, new_cube_state);

            if new_cube_state == CubeState::Active {
                new_state.insert((x, y, z));
            } 
        }
        
        self.min_max_x.0 = new_state.iter().min_by_key(|(x, _, _)| x).unwrap().0;
        self.min_max_x.1 = new_state.iter().max_by_key(|(x, _, _)| x).unwrap().0+1;

        self.min_max_y.0 = new_state.iter().min_by_key(|(_, y, _)| y).unwrap().1;
        self.min_max_y.1 = new_state.iter().max_by_key(|(_, y, _)| y).unwrap().1+1;

        self.min_max_z.0 = new_state.iter().min_by_key(|(_, _, z)| z).unwrap().2;
        self.min_max_z.1 = new_state.iter().max_by_key(|(_, _, z)| z).unwrap().2+1;
        self.activate_cubes = new_state;
    }
}

impl Iterator for PocketDimension {
    type Item = PocketDimension;

    fn next(&mut self) -> Option<PocketDimension> {
        self.tick();

        Some(self.clone())
    }
}

fn parse_file(file_name: &str) -> PocketDimension {
    let input = fs::read_to_string(file_name).expect("file not found!");
    let dimensional_cubestates = input.lines().map(|l| l.chars().map(|c| {
            match c {
                '#' => CubeState::Active,
                '.' => CubeState::Inactive,
                 _  => CubeState::Inactive,
            }
        }).collect::<Vec<CubeState>>())
        .collect::<Vec<Vec<CubeState>>>();
    
    let size_x = dimensional_cubestates.first().unwrap().len() as isize;
    let size_y = dimensional_cubestates.len() as isize;
    let size_z = 1;

    PocketDimension {
        activate_cubes: dimensional_cubestates.into_iter()
                                              .flatten()
                                              .enumerate()
                                              .filter_map(|(i, s)| {
                                                if s == CubeState::Active {
                                                    Some((i as isize % size_x,
                                                         (i as isize / size_x) % size_y,
                                                         0))
                                                }
                                                else {
                                                    None
                                                }
                                              }).collect(),
        min_max_x: (0, size_x),
        min_max_y: (0, size_y),
        min_max_z: (0, size_z),
    }
}

impl fmt::Display for CubeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CubeState::Active   => write!(f, "◼"),
            CubeState::Inactive => write!(f, "◻"),
        }
    }
}

impl fmt::Display for PocketDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size: (x:{:?}, y:{:?}, z:{:?})\n",
                  self.min_max_x, self.min_max_y, self.min_max_z)?;
        for z in self.min_max_z.0..self.min_max_z.1 {
            write!(f, "z={}\n", z)?;
            for y in self.min_max_y.0..self.min_max_y.1 {
                for x in self.min_max_x.0..self.min_max_x.1 {
                    write!(f, "{}", self.index(x, y, z))?;
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_part1() {
        let mut pocket_dimension = parse_file("dat/day17_example.txt");
        println!("Initial state: {}", pocket_dimension);
        
        pocket_dimension = pocket_dimension.skip(5).nth(0).unwrap();
        println!("Final state: {}", pocket_dimension);
        assert_eq!(pocket_dimension.activate_cubes.len(), 112);
    }
}