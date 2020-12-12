use std::path::Path;
use std::fs;

fn main() {
    let adapters_string = fs::read_to_string(Path::new("adapters.txt"))
        .expect("Adapters file not found!");

    let mut joltages: Vec<usize> = adapters_string.split("\n")
        .map(|line| line.parse::<usize>().expect("Not a number!"))
        .collect();

    joltages.sort();
    joltages.insert(0, 0);
    let last_joltage = joltages.last().unwrap().clone();
    joltages.push(last_joltage + 3);

    let mut joltage_jumps: Vec<usize> = vec![];

    joltage_jumps.push(joltages[0]);

    for (joltage1, joltage2) in joltages.iter().zip(joltages.iter().skip(1)) {
        joltage_jumps.push(joltage2 - joltage1)
    }

    joltage_jumps.push(3);

    let joltage_groups = build_joltage_groups(joltage_jumps, joltages.clone());

    println!("{:?}", joltage_groups);

    let combinations = joltage_groups.iter().map(|group| check_valid_joltage_combinations_brute_force(group))
    .fold(1u128, |acc, val| {acc * val as u128})
    ;

    println!("{:?}",combinations);
}

fn build_joltage_groups(joltage_jumps: Vec<usize>, joltages: Vec<usize>) -> Vec<Vec<usize>> {
    let mut output = vec![];
    let mut new_group: Vec<usize> = vec![];

    for (jump, joltage) in joltage_jumps.into_iter().zip(joltages) {
        if jump == 3 && new_group.len() != 1 {
            output.push(new_group.clone());
            new_group.clear();
        }
        new_group.push(joltage)
    }

    output
}

fn check_valid_joltage_combinations_brute_force(joltages: &Vec<usize>) -> usize {
    let mut valid_joltage_combinations = 0;
    let total_length = joltages.len();

    for combinator in 0..2usize.pow((total_length - 2) as u32) {
        let selected_joltages: Vec<usize> = joltages.iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if index == 0 || index == total_length - 1 || 2usize.pow((index - 1) as u32) & combinator != 0 {
                return Some(*value)
            } else {
                None
            } 
        })
        .collect();

        if are_joltages_valid(selected_joltages) {
            valid_joltage_combinations += 1;
        }
    }

    valid_joltage_combinations
}

fn are_joltages_valid(joltages: Vec<usize>) -> bool {
    for (joltage1, joltage2) in joltages.iter().zip(joltages.iter().skip(1)) {
        match joltage2 - joltage1 {
            1 | 2 | 3 => continue,
            _ => return false,
        };
    }
    true
}