use std::collections::{HashMap, HashSet};
use std::{path::Path};
use std::fs;

fn main() {
    let ticket_data_string = fs::read_to_string(Path::new("ticket_data.txt"))
        .expect("Ticket data file not found!");

    let checker = TicketChecker::new(ticket_data_string);

    let description_names = vec![
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
    ];

    println!("{:?}", checker.get_product_of_personal_ticket_values_for_names(description_names));
}


struct TicketChecker {
    validators: Vec<Validator>,
    personal_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

impl TicketChecker {
    fn new(ticket_data_string: String) -> Self {
        let mut ticket_data_parts =  ticket_data_string.split("\n\n");

        let validators = ticket_data_parts
            .next()
            .and_then(|p| Some(p.split("\n").map(|v| Validator::new(v)))).expect("Couldn't build validator!")
            .collect();

        let personal_ticket = ticket_data_parts
            .next()
            .and_then(|p| p.split("\n").nth(1))
            .and_then(|line| Some(Ticket::new(line)))
            .expect("Invalid Personal Ticket!");

        let nearby_tickets = ticket_data_parts
            .next()
            .and_then(|p| Some(p.split("\n").skip(1).map(|v| Ticket::new(v)))).expect("Couldn't build validator!")
            .collect();

        Self {
            validators,
            personal_ticket,
            nearby_tickets
        }
    }

    fn get_valid_tickets(&self) -> Vec<&Ticket> {
        let mut output = vec![];

        for ticket in &self.nearby_tickets {
            let mut valid_numbers = HashSet::new();

            for validator in &self.validators {
                let new_valid_numbers = validator.validate_ticket(&ticket);

                valid_numbers.extend(&new_valid_numbers);
            }

            let invalid_ticket_numbers = ticket.get_numbers().difference(&valid_numbers);

            if invalid_ticket_numbers.count() == 0 {
                output.push(ticket);
            }
        }
        output
    }

    fn get_validator_with_name(&self, name: &str) -> &Validator {
        self.validators.iter().find(|v| v.field_name == name).expect("No validator with this name")
    }

    fn get_valid_ticket_positions_for_name(&self, name: &str) -> HashSet<usize> {
        let validator = self.get_validator_with_name(name);
        let valid_tickets =  self.get_valid_tickets();

        (0..self.personal_ticket.get_numbers().len())
            .filter(|index| validator.validate_position(*index, &valid_tickets))
            .collect()
    }

    fn get_validator_field_names(&self) -> Vec<&str> {
        self.validators.iter().map(|v| v.get_field_name()).collect()
    }

    fn get_valid_ticket_positions_for_names(&self) -> HashMap<String, HashSet<usize>>  {
        let names = self.get_validator_field_names();
        names.iter().map(|name| (name.to_string(), self.get_valid_ticket_positions_for_name(name))).collect()
    }

    fn determine_positions_for_names(&self) -> HashMap<String, usize> {
        let mut valid_positions = self.get_valid_ticket_positions_for_names();
        let mut output = HashMap::new();
        let total_positions = self.personal_ticket.get_numbers().len();
        
        while valid_positions.len() > 0 {
            if let Some(val) = (0..total_positions)
                .find(
                    |p| valid_positions.values().filter(|vp| vp.get(p).is_some()).count() == 1
                ) {
                    let key = valid_positions.iter()
                        .find(|(_, positions)| positions.get(&val).is_some())
                        .unwrap()
                        .0.clone();

                    valid_positions.remove(&key);

                    output.insert(key, val);
                }
            else {
                panic!("No single position found! Current valid_positions: {:?}, current output: {:?}", valid_positions, output);
            }
        }

        output
    }

    fn get_product_of_personal_ticket_values_for_names(&self, names: Vec<&str>) -> usize {
        let name_positions = self.determine_positions_for_names();
        let personal_ticket_values = self.personal_ticket.get_numbers_ordered();

        names.iter()
            .map(|n| name_positions.get(&n.to_string()).unwrap())
            .map(|pos| personal_ticket_values[*pos])
            .fold(1, |acc, val| acc * val)
    }
}

struct Validator {
    field_name: String,
    ranges: Vec<ValidatorRange>
}

impl Validator {
    fn new(validator_string: &str) -> Self {
        let mut string_parts = validator_string.split(":");
        let field_name = string_parts.next().unwrap().trim().to_string();
        let ranges_string = string_parts.next().expect("No range for validation line!").trim();
        let ranges = ranges_string.split("or").map(|s| {
            let mut range_parts = s.trim().split("-");
            ValidatorRange::new(
                range_parts.next().and_then(|p| p.parse::<usize>().ok()).expect("Invalid Range Part!"), 
                range_parts.next().and_then(|p| p.parse::<usize>().ok()).expect("Invalid Range Part!")
            )
        }).collect();

        Self {
            field_name,
            ranges
        }
    }

    fn validate_ticket(&self, ticket: &Ticket) -> HashSet<usize> {
        let mut output = HashSet::new();

        for num in ticket.get_numbers() {
            if self.ranges.iter().any(|r| r.validate(*num)) {
                output.insert(*num);
            }
        }

        output
    }

    fn validate_position(&self, pos: usize, tickets: &Vec<&Ticket>) -> bool {
        tickets.iter()
            .map(|t| t.get_numbers_ordered().iter().nth(pos).unwrap())
            .all(|n| self.ranges.iter().any(|r| r.validate(*n)))
    }

    fn get_field_name(&self) -> &str {
        &self.field_name
    }
}

#[derive(Debug)]
struct ValidatorRange {
    start: usize,
    end: usize
}

impl ValidatorRange {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }

    fn validate(&self, num: usize) -> bool {
        num >= self.start && num <= self.end
    }
}

#[derive(Debug)]
struct Ticket {
    field_numbers: HashSet<usize>,
    numbers_ordered: Vec<usize>
}

impl Ticket {
    fn new(ticket_string: &str) -> Self {
        Self {
            field_numbers: ticket_string.split(',').map(|s| s.parse::<usize>().expect("Ticket number parse error!")).collect(),
            numbers_ordered: ticket_string.split(',').map(|s| s.parse::<usize>().expect("Ticket number parse error!")).collect()
        }
    }

    fn get_numbers(&self) -> &HashSet<usize> {
        &self.field_numbers
    }

    fn get_numbers_ordered(&self) -> &Vec<usize> {
        &self.numbers_ordered
    }
}

