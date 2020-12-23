use std::fs;
use std::fmt;
use std::collections::HashSet;

use itertools::Itertools;

pub fn solve() {
    let mut pocket_dimension = parse_file("dat/day17.txt");
    let mut pocket_dimension4 = pocket_dimension.clone();
    pocket_dimension4.add_dimension();
    // println!("Initial state: {}", pocket_dimension);
    
    pocket_dimension = pocket_dimension.skip(5).nth(0).unwrap();
    // println!("Final state: {}", pocket_dimension);
    println!("iter 6: {}", pocket_dimension.activate_cubes.len());

    pocket_dimension4 = pocket_dimension4.skip(5).nth(0).unwrap();
    // println!("Final state: {}", pocket_dimension4);
    println!("iter 6: {}", pocket_dimension4.activate_cubes.len());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

#[derive(Debug, Clone,)]
struct PocketDimension {
    activate_cubes: HashSet<Vec<isize>>,
}

impl PocketDimension {
    fn index(&self, i: &Vec<isize>) -> CubeState {
        if self.activate_cubes.contains(i) {CubeState::Active}
        else {CubeState::Inactive}
    }

    fn get_nr_dimensions(&self) -> usize {
        self.activate_cubes.iter().nth(0).unwrap().len()
    }

    fn get_dimension_sizes(&self) -> Vec<(isize, isize)> {
        (0..self.get_nr_dimensions())
            .map(|i| self.activate_cubes.iter().map(|v| v[i]).minmax().into_option().unwrap())
            .collect()
    }

    fn add_dimension(&mut self) {
        self.activate_cubes = self.activate_cubes
            .iter()
            .map(|i| {
                let mut ni = i.clone();
                ni.push(0);
                ni
            }).collect();
    }

    fn get_neighbor_indices(&self, i: &Vec<isize>) -> Vec<Vec<isize>> {
        (0..self.get_nr_dimensions())
            .map(|_| -1..=1)
            .multi_cartesian_product()
            .filter(|o| !o.iter().all(|&i| i == 0)) // ignore self index
            // .inspect(|x| println!("{:?}", x))
            .map(|v| v.iter().zip(i.iter()).map(|(a, b)| a+b).collect::<Vec<isize>>())
            .collect()
    }

    fn neighbor_count(&self, i: &Vec<isize>, state:CubeState) -> usize {
        self.get_neighbor_indices(&i).iter()
                 .filter(|o| self.index(o) == state)
                //  .inspect(|x| println!("{:?}", x))
                 .count()
    }

    fn tick(&mut self) {
        self.activate_cubes = self.activate_cubes.iter()
            .flat_map(|i| self.get_neighbor_indices(i))
            .unique()
            // .inspect(|x| println!("{:?}", x))
            .fold(HashSet::<Vec<isize>>::new(), |mut acc, i| {
                let ct = self.index(&i);
                let anc = self.neighbor_count(&i, CubeState::Active);
                match (ct, anc) {
                    (CubeState::Active, x) if x == 2 || x == 3 => acc.insert(i),
                    (CubeState::Inactive, 3) => acc.insert(i),
                    _ => false,
                };
                acc
            });
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

    PocketDimension {
        activate_cubes: dimensional_cubestates
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, s)| {
            if s == CubeState::Active {
                Some(vec![i as isize % size_x,(i as isize / size_x) % size_y,0])
            }
            else {
                None
            }
            }).collect(),
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

// TODO: Do something beter for this
impl fmt::Display for PocketDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n_dimensions = self.get_nr_dimensions();
        let dimension_sizes: Vec<(isize, isize)> = self.get_dimension_sizes();
        write!(f, "N dimensions: {}, sizes: {:?})\n",
                  n_dimensions, dimension_sizes)?;
        dimension_sizes
            .iter()
            .map(|d| (d.0..=d.1))
            .multi_cartesian_product()
            .enumerate()
            .for_each(|(i, x)| {
                write!(f, "i: {}, ind: {:?} -> {}\n", i, x, self.index(&x)).unwrap();
            });
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

    #[ignore]
    #[test]
    fn test_day17_part2() {
        let mut pocket_dimension4 = parse_file("dat/day17_example.txt");
        pocket_dimension4.add_dimension();
        println!("Initial state: {}", pocket_dimension4);
        
        pocket_dimension4 = pocket_dimension4.skip(5).nth(0).unwrap();
        println!("Final state: {}", pocket_dimension4);
        assert_eq!(pocket_dimension4.activate_cubes.len(), 848);
    }
}