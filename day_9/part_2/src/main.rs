use std::path::Path;
use std::fs;

fn main() {
    let number_string = fs::read_to_string(Path::new("numbers.txt"))
        .expect("Numbers file not found!");

    let numbers: Vec<u64> = number_string.split("\n")
        .map(|line| line.parse::<u64>()
        .expect("Invalid Number"))
        .collect();

    let target_number= 1398413738;

    'outer: for (index1, number1) in numbers.iter().enumerate() {
        let mut sum = *number1;

        for (index2, number2) in numbers.iter().skip(index1 +1).enumerate() {
            sum += number2;

            if sum == target_number {
                println!("{} {}", index1, index2 + index1);

                let min = numbers[index1..(index2 + index1)].iter().min().expect("No Min!");
                let max = numbers[index1..(index2 + index1)].iter().max().expect("No Max!");

                println!("{}", min + max);
                break 'outer
            }
        }
    }
}
