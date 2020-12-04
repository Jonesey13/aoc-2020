use std::{path::Path};
use std::fs;
use regex::Regex;

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
            "byr" => validate_numerical_range(value, 1920, 2002),
            "iyr" => validate_numerical_range(value, 2010, 2020),
            "eyr" => validate_numerical_range(value, 2020, 2030),
            "hgt" => validate_height(value),
            "hcl" => validate_hair_color(value),
            "ecl" => validate_eye_color(value),
            "pid" => validate_digits(value, 9),
            _ => continue     
        };

        if validated {
            validated_prefix_count += 1;
        }
    }

    validated_prefix_count == 7
}

fn validate_digits(value: &str, string_length: usize) -> bool {
    if value.len() != string_length {
        false
    } else {
        let regex_string = Regex::new(&("[0-9]".to_string() + &format!("{{{}}}", string_length)))
            .expect("Invalid Regex!");
        regex_string.is_match(value)
    }
}

fn validate_eye_color(value: &str) -> bool {
    let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.iter().find(|word| value == **word).is_some()
}

fn validate_hair_color(value: &str) -> bool {
    let prefix = &value[0..1];

    match prefix {
        "#" => validate_characters_or_digits(&value[1..], 6),
        _ => false
    }
}

fn validate_characters_or_digits(value: &str, string_length: usize) -> bool {
    if value.len() != string_length {
        false
    } else {
        let regex_string = Regex::new(&("[a-z0-9]".to_string() + &format!("{{{}}}", string_length)))
            .expect("Invalid Regex!");
        regex_string.is_match(value)
    }
}

fn validate_height(value: &str) -> bool {
    let length = value.len();
    let postfix = &value[length-2..];

    match postfix {
        "cm" => validate_numerical_range(&value[0..length-2], 150, 193),
        "in" => validate_numerical_range(&value[0..length-2], 59, 76),
        _ => false
    }
}

fn validate_numerical_range(value: &str, min: usize, max: usize) -> bool {
    let parsed_value = value.parse::<usize>();

    if let Ok(value) = parsed_value {
        value >= min && value <= max
    } else {
        false
    }
}