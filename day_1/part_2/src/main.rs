use std::{path::Path, fs};
use itertools::{self, iproduct};

fn main() {
    let expenses_string = fs::read_to_string(Path::new("expenses.txt"))
        .expect("Expenses file not found!");

    let expenses: Vec<usize> = expenses_string.split("\n")
        .map(|e| e.parse::<usize>().expect("Expense is not a number!"))
        .collect();

    for ((i1, expense1), (i2, expense2), (i3, expense3)) in iproduct!(
        expenses.clone().into_iter().enumerate(), 
        expenses.clone().into_iter().enumerate(),
        expenses.clone().into_iter().enumerate()
    ) {
        if i1 == i2 || i1 == i3 || i2 == i3 { continue }
        else if expense1 + expense2 + expense3 == 2020 {
            println!("{} {} {} {} {} {}", i1, expense1, i2, expense2, i3, expense3);
            println!("{}", expense1 * expense2 * expense3);
            break
        }
    }
}
