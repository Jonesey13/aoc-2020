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

    for (i, answer) in answers.enumerate() {
        let characters = answer.chars();

        if i == 0 {
            for character in characters {
                answer_codes.insert(character);
            }
        } else {
            let mut current_answer_codes = HashSet::new();

            for character in characters {
                current_answer_codes.insert(character);
            }

            answer_codes = answer_codes.intersection(&current_answer_codes).cloned().collect::<HashSet<char>>();
        }
    }

    answer_codes.len()
}
