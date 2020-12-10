use std::path::Path;
use std::fs;

fn main() {
    let number_string = fs::read_to_string(Path::new("numbers.txt"))
        .expect("Numbers file not found!");

    let numbers: Vec<u64> = number_string.split("\n")
        .map(|line| line.parse::<u64>()
        .expect("Invalid Number"))
        .collect();

    'outer: for (i, number) in numbers.iter().skip(25).enumerate() {
        let true_index = i + 25;

        let candidate_numbers = &numbers[i..true_index];

        for (index1, number1) in candidate_numbers.iter().enumerate() {
            for (index2, number2) in candidate_numbers.iter().enumerate() {
                if index1 == index2 {
                    continue
                } else if number1 + number2 == *number {
                    continue 'outer
                }
            }
        }

        println!("{} {}", true_index, number);

        break
    }
}
