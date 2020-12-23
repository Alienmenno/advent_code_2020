use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

pub fn solve() {
    let ticket_data = parse_file("dat/day16.txt");
    // println!("r: {:?}", ticket_data);

    let invalid_tickets = find_invalid_fields_in_tickets(&ticket_data);
    let sum_invalid_fields = invalid_tickets.iter().map(|(_, v)| v).sum::<usize>();
    println!("Result: {:?}", sum_invalid_fields);
    
    let invalid_ticket_ids = invalid_tickets.into_iter().map(|(i, _)| i).collect::<Vec<usize>>();
    let valid_tickets = determine_valid_tickets(&ticket_data, invalid_ticket_ids);
    // println!("Valid Tickets: {:?}", valid_tickets.len());

    let field_order = determine_ticket_field_order(&ticket_data, &valid_tickets);
    // println!("Field order: {:?}", field_order);

    let depart_time = determine_departure_time(&field_order, &ticket_data.my_ticket);
    println!("Depature Time: {:?}", depart_time);
}

fn find_invalid_fields_in_tickets(ticket_data: &TicketData) -> Vec<(usize, usize)> {

    ticket_data.other_tickets.iter().enumerate().filter_map(|(i, t)| {
        t.fields.iter().filter_map(|tv| {
            if !ticket_data.rules.iter().any(|(_, v)| {
                v.iter().any(|(min, max)| min <= tv && tv <= max)
            }) {
                Some((i, tv.clone()))
            }
            else {
                None
            }
        }).nth(0)
    }).collect()
}

fn determine_valid_tickets(ticket_data: &TicketData, invalid_ticket_ids: Vec<usize>) -> Vec<Ticket> {
    ticket_data.other_tickets.iter().enumerate().filter_map(|(i, t)| {
        if !invalid_ticket_ids.contains(&i) {Some(t.clone())}
        else {None}
    }).collect::<Vec<Ticket>>()
}

fn determine_ticket_field_order(ticket_data: &TicketData, valid_tickets: &Vec<Ticket>) -> Vec<(String, usize)> {
    let mut possible_field_orders: Vec<(String, Vec<usize>)> = Vec::new();
    for (rule, vals) in &ticket_data.rules {
        let v = (0..ticket_data.rules.len()).filter(|&i| {
            valid_tickets.iter()
                         .all(|ticket| {
                            vals.iter()
                            .any(|(min, max)| {
                                min <= &ticket.fields[i] && &ticket.fields[i] <= max
                            })
                         })
            }).collect();
        possible_field_orders.push((rule.clone(), v));
    }
    possible_field_orders.sort_by_key(|(_, indexs)| indexs.len());
    // println!("Possbible values: {:?}", possible_field_orders);

    let mut determined_indices: HashSet::<usize> = HashSet::new();
    possible_field_orders.iter().map(|(r, vs)| {
        let v = vs.into_iter().filter(|v| !determined_indices.contains(&v)).nth(0).unwrap_or(&0);
        determined_indices.insert(*v);
        (r.to_string(), *v)
    }).collect::<Vec<(String, usize)>>()
}

fn determine_departure_time(field_indices: &Vec<(String, usize)>, ticket: &Ticket) -> usize {
    let dep = "departure";
    field_indices.iter().filter_map(|(field_name, i)| {
        if field_name.contains(&dep) {Some(ticket.fields[*i])}
        else{None}
    }).product()
}

#[derive(Debug, Clone,)]
struct Ticket {
    fields: Vec<usize>,
}

#[derive(Debug, Clone,)]
struct TicketData {
    rules: HashMap<String, Vec<(usize, usize)>>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse_file(file_name: &str) -> TicketData {
    let input = fs::read_to_string(file_name).expect("file not found!");
    let ticket_data_lines = input.lines().map(String::from).collect::<Vec<String>>();
    let ticket_data_parts = ticket_data_lines.split(|l| l.is_empty()).collect::<Vec<&[String]>>();

    TicketData {
        rules: parse_ticket_rules(&ticket_data_parts[0]),
        my_ticket: parse_tickets(&ticket_data_parts[1])[0].clone(),
        other_tickets: parse_tickets(&ticket_data_parts[2]),
    }
}
    
fn parse_ticket_rules(ticket_data: &[String]) -> HashMap::<String, Vec<(usize, usize)>> {
    let re = Regex::new(r"^([a-z| ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
    ticket_data.iter()
               .map(|rule| re.captures(rule).unwrap())
               .fold(HashMap::<String, Vec<(usize, usize)>>::new(), |mut acc, rule| {
                    let name = rule.get(1).unwrap().as_str().to_string();
                    let range_values = rule.iter()
                                           .skip(2)
                                           .map(|m| m.unwrap().as_str().parse::<usize>().unwrap())
                                           .collect::<Vec<usize>>();
                    acc.insert(name, vec![(range_values[0], range_values[1]), (range_values[2], range_values[3])]);
                    acc
                })
}

fn parse_tickets(ticket_data: &[String]) -> Vec<Ticket> {
    ticket_data.iter()
               .skip(1) // Skip header
               .map(|t|  Ticket {
                   fields: t.split(',')
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                }).collect::<Vec<Ticket>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        let ticket_data = parse_file("dat/day16_example.txt");
        println!("r: {:?}", ticket_data);
    
        let r = find_invalid_fields_in_tickets(&ticket_data).iter().map(|(_, v)| v).sum::<usize>();
        assert_eq!(r, 71);
    }

    #[test]
    fn test_day16_part2() {
        let ticket_data = parse_file("dat/day16_example2.txt");
        println!("r: {:?}", ticket_data);
    
        let invalid_tickets = find_invalid_fields_in_tickets(&ticket_data);
        let sum_invalid_fields = invalid_tickets.iter().map(|(_, v)| v).sum::<usize>();
        println!("Result: {:?}", sum_invalid_fields);
        
        let invalid_ticket_ids = invalid_tickets.into_iter().map(|(i, _)| i).collect::<Vec<usize>>();
        let mut valid_tickets = determine_valid_tickets(&ticket_data, invalid_ticket_ids);
        valid_tickets.push(ticket_data.my_ticket.clone()); // Add my own ticket
        println!("Valid Tickets: {:?}", valid_tickets);
    
        let field_order = determine_ticket_field_order(&ticket_data, &valid_tickets);
        println!("Field order: {:?}", field_order);
    
        let depart_time = determine_departure_time(&field_order, &ticket_data.my_ticket);
        assert_eq!(depart_time, 12);
    }
}