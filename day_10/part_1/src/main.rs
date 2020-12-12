use std::path::Path;
use std::fs;

fn main() {
    let adapters_string = fs::read_to_string(Path::new("adapters.txt"))
        .expect("Adapters file not found!");

    let mut joltages: Vec<usize> = adapters_string.split("\n")
        .map(|line| line.parse::<usize>().expect("Not a number!"))
        .collect();

    joltages.sort();

    let mut num_1_jumps = 0;
    let mut num_3_jumps = 1;

    match joltages[0] {
        1 => num_1_jumps += 1,
        3 => num_3_jumps += 1,
        _ => ()
    };

    for (joltage1, joltage2) in joltages.iter().zip(joltages.iter().skip(1)) {
        match joltage2 - joltage1 {
            1 => num_1_jumps += 1,
            3 => num_3_jumps += 1,
            _ => ()
        };
    }

    println!("{}", num_1_jumps * num_3_jumps)
}
