use std::{path::Path};
use std::fs;

fn main() {
    let passport_string = fs::read_to_string(Path::new("passports.txt"))
        .expect("Passports file not found!");

    let passports = passport_string.split("\n\n");

    println!("{}", passports.filter(|p| validate_passport_string(p)).count())
}

fn validate_passport_string(passsport_string: &str) -> bool {
    let sections: Vec<&str> = passsport_string.split("\n")
        .flat_map(|section| section.split(" "))
        .collect();

    let prefixes_and_values = sections.into_iter()
        .map(|section| {
            let mut parts = section.split(":");
            (parts.next().expect("No prefix"), parts.next().expect("No value"))
        });

    let mut validated_prefix_count = 0;

    for (prefix, value) in prefixes_and_values {
        let validated: bool = match prefix {
            "byr" => true,
            "iyr" => true,
            "eyr" => true,
            "hgt" => true,
            "hcl" => true,
            "ecl" => true,
            "pid" => true,
            _ => continue     
        };

        if validated {
            validated_prefix_count += 1;
        }
    }

    validated_prefix_count == 7
}