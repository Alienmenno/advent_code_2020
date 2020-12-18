use std::fs;
use std::iter;

pub fn solve() {
    let bus_notes = parse_bus_schedule_file("dat/day13.txt");
    println!("{:?}", bus_notes);

    let r = find_earliest_bus_depature(bus_notes.earliest_depature_time, &bus_notes.available_busses);
    println!("Result: {}", r.0 * r.1.id);

    let r = find_earliest_bus_sequence_time(&bus_notes.available_busses);
    println!("result: {}", r);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bus {
    id: usize,
    offset: usize,
}

#[derive(Debug)]
struct BusNotes {
    earliest_depature_time: usize,
    available_busses: Vec<Bus>,
}

fn find_earliest_bus_depature(target_time: usize, busses: &Vec<Bus>) -> (usize, Bus) {
    let r = busses.iter().map(|b| {
        ((target_time as f64 / b.id as f64).ceil() as usize * b.id, *b)
    }).min_by(|(t1, _), (t2, _)| t1.cmp(t2)).unwrap();
    return (r.0 - target_time, r.1);
}

/* Using Linear Diophantine: One equation to find interger solutions the system
   of linear equations. This solution works for any combination of bus IDs
   as long as they are all prime numbers. The reason this only works for primes
   is that their greatest common divisor is 1 so defining v and u as being a / 1
   and b / 1 respectively
*/
fn find_earliest_bus_sequence_time(busses: &Vec<Bus>) -> usize {
    let first_bus = *busses.first().unwrap();
    let mut found_busses: Vec<Bus> = Vec::new();
    found_busses.push(first_bus);
    let mut target = first_bus.id;

    for bus in busses.iter().skip(1) {
        let mut k = 0;
        target = iter::repeat_with(|| {
            let tmp = target + (k * found_busses.iter().map(|b| b.id).product::<usize>());
            k += 1;
            tmp
        }).skip_while(|t| {
            (t + bus.offset) % bus.id != 0
        }).nth(0).unwrap();

        found_busses.push(*bus);
        println!("found_busses: {:?}, target: {}", found_busses, target);
    }

    return target;
}

#[allow(dead_code)]
fn find_earliest_bus_sequences_brute_force(busses: &Vec<Bus>) -> usize {
    let mut sorted_busses = busses.clone();
    sorted_busses.sort_by(|Bus{id: id1, offset:_}, Bus{id:id2, offset:_}| id2.cmp(id1));
    let other_busses = sorted_busses.split_off(2);
    println!("{:?}", sorted_busses);
    let mut i = 1;
    let tgen = iter::repeat_with(|| {let tmp = sorted_busses[0].id*i - sorted_busses[0].offset; i += 1; tmp});
    let t = tgen.skip_while(|x| {
        if (x + sorted_busses[1].offset) % sorted_busses[1].id == 0 {
            println!("Checking: {}", x);
            !other_busses.iter().all(|Bus{id: i, offset: o}| (x + o) % i == 0)
        }
        else {true}
    }).nth(0).unwrap();
    println!("t: {:?}", t);

    return t;
}

fn parse_bus_schedule_file(file_name: &str) -> BusNotes {
    let input = fs::read_to_string(file_name).expect("file not found!");
    let notes = input.split_whitespace().collect::<Vec<&str>>();

    return BusNotes {
        earliest_depature_time: notes.first().unwrap().parse().unwrap(),
        available_busses: notes.last().unwrap().split(',').enumerate()
                               .filter(|(_, b)| b.parse::<usize>().is_ok())
                               .map(|(i, b)| Bus{id:b.parse::<usize>().unwrap(), offset: i}).collect(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1() {
        let bus_notes = parse_bus_schedule_file("dat/day13_example.txt");
        println!("{:?}", bus_notes);
    
        let r = find_earliest_bus_depature(bus_notes.earliest_depature_time, &bus_notes.available_busses);
        assert_eq!(r.0 * r.1.id, 295);
    }

    #[test]
    fn test_day13_part2() {
        let bus_notes = parse_bus_schedule_file("dat/day13_example.txt");
        println!("{:?}", bus_notes);
    
        let r = find_earliest_bus_sequences_brute_force(&bus_notes.available_busses);
        assert_eq!(r, 1068781);
        let r = find_earliest_bus_sequence_time(&bus_notes.available_busses);
        assert_eq!(r, 1068781);
    }

    #[test]
    fn test_day13_part2_1() {
        let busses = vec![Bus{id:17, offset: 0}, Bus{id:13, offset: 2}, Bus{id:19, offset: 3}];
        println!("{:?}", busses);
    
        let r = find_earliest_bus_sequences_brute_force(&busses);
        assert_eq!(r, 3417);
        let r = find_earliest_bus_sequence_time(&busses);
        assert_eq!(r, 3417);
    }

    #[test]
    fn test_day13_part2_2() {
        let busses = vec![Bus{id:67, offset: 0}, Bus{id:7, offset: 1}, Bus{id:59, offset: 2}, Bus{id:61, offset: 3}];
        println!("{:?}", busses);
    
        let r = find_earliest_bus_sequences_brute_force(&busses);
        assert_eq!(r, 754018);
        let r = find_earliest_bus_sequence_time(&busses);
        assert_eq!(r, 754018);
    }

    #[test]
    fn test_day13_part2_3() {
        let busses = vec![Bus{id:67, offset: 0}, Bus{id:7, offset: 2}, Bus{id:59, offset: 3}, Bus{id:61, offset: 4}];
        println!("{:?}", busses);
    
        let r = find_earliest_bus_sequences_brute_force(&busses);
        assert_eq!(r, 779210);
        let r = find_earliest_bus_sequence_time(&busses);
        assert_eq!(r, 779210);
    }

    #[test]
    fn test_day13_part2_4() {
        let busses = vec![Bus{id:67, offset: 0}, Bus{id:7, offset: 1}, Bus{id:59, offset: 3}, Bus{id:61, offset: 4}];
        println!("{:?}", busses);
    
        let r = find_earliest_bus_sequences_brute_force(&busses);
        assert_eq!(r, 1261476);
        let r = find_earliest_bus_sequence_time(&busses);
        assert_eq!(r, 1261476);
    }

    #[test]
    fn test_day13_part2_5() {
        let busses = vec![Bus{id:1789, offset: 0}, Bus{id:37, offset: 1}, Bus{id:47, offset: 2}, Bus{id:1889, offset: 3}];
        println!("{:?}", busses);
    
        let r = find_earliest_bus_sequences_brute_force(&busses);
        assert_eq!(r, 1202161486);
        let r = find_earliest_bus_sequence_time(&busses);
        assert_eq!(r, 1202161486);
    }
}