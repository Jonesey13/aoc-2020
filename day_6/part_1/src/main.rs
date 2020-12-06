use std::collections::HashSet;
use std::{path::Path};
use std::fs;

fn main() {
    let answers_string = fs::read_to_string(Path::new("answers.txt"))
        .expect("Answers file not found!");

    let groups = answers_string.split("\n\n");

    let mut answer_total = 0;

    for group in groups {
        answer_total += get_total_answers_for_group(group);
    }

    println!("{}", answer_total)
}

fn get_total_answers_for_group(group: &str) -> usize {
    let mut answer_codes = HashSet::new();

    let answers = group.split("\n");

    for answer in answers {
        let characters = answer.chars();

        for character in characters {
            answer_codes.insert(character);
        }
    }

    answer_codes.len()
}
