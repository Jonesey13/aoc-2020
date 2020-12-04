use std::path::Path;
use std::fs;


fn main() {
    let passwords_string = fs::read_to_string(Path::new("passwords.txt"))
        .expect("Expenses file not found!");

    let passwords_and_criteria: Vec<(&str, &str)> = passwords_string
        .split("\n")
        .map(|line| {
            let mut password_and_criteria = line.split(": ");
            (password_and_criteria.next().expect("No Criteria"), password_and_criteria.next().expect("No Password"))
        })
        .collect();

    let mut sum = 0;

    for (criteria, password) in passwords_and_criteria {
        let mut criteria_parts = criteria.split(" ");
        let (mut occurance_parts, letter) = (
            criteria_parts.next().expect("No Occurance Data").split("-"), 
            criteria_parts.next().expect("No Letter Data").chars().next().unwrap()
        );
        let (position_1, position_2) = (
            occurance_parts.next().expect("No position 1").parse::<usize>().expect("position 1 not a number"),
            occurance_parts.next().expect("No position 2").parse::<usize>().expect("position 2 not a number")
        );

        if check_password_validity(password, letter, position_1, position_2) {
            sum += 1
        }
    }

    println!("{}", sum)
}

fn check_password_validity(password: &str, letter: char, position_1: usize, position_2: usize) -> bool {
    (password.chars().nth(position_1 - 1).expect("Missing Letter For Position") == letter)
        ^ (password.chars().nth(position_2 - 1).expect("Missing Letter For Position") == letter)
}