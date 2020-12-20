use std::collections::HashSet;
use std::{path::Path};
use std::fs;

fn main() {
    let ticket_data_string = fs::read_to_string(Path::new("ticket_data.txt"))
        .expect("Ticket data file not found!");

    let checker = TicketChecker::new(ticket_data_string);

    let error_rate = checker.check_nearby_tickets();

    println!("{}", error_rate);
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

    fn check_nearby_tickets(&self) -> usize {
        let mut output = 0;

        for ticket in &self.nearby_tickets {
            let mut valid_numbers = HashSet::new();

            for validator in &self.validators {
                let new_valid_numbers = validator.validate_ticket(&ticket);

                valid_numbers.extend(&new_valid_numbers);
            }

            let invalid_ticket_numbers = ticket.get_numbers().difference(&valid_numbers);

            output += invalid_ticket_numbers.clone().fold(0, |acc, val| acc + val) as usize;
        }

        output
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

struct Ticket {
    field_numbers: HashSet<usize>
}

impl Ticket {
    fn new(ticket_string: &str) -> Self {
        Self {
            field_numbers: ticket_string.split(',').map(|s| s.parse::<usize>().expect("Ticket number parse error!")).collect()
        }
    }

    fn get_numbers(&self) -> &HashSet<usize> {
        &self.field_numbers
    }
}

