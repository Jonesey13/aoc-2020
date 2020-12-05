use std::{path::Path};
use std::fs;

fn main() {
    let seats_string = fs::read_to_string(Path::new("seats.txt"))
        .expect("Seats file not found!");

    let seats = seats_string.split("\n");

    let mut seat_ids: Vec<usize> = seats.map(|seat| calculate_seat_id(seat)).collect();
    seat_ids.sort();

    for (prev_id, next_id) 
        in seat_ids.clone().into_iter()
            .zip(seat_ids.clone().into_iter().skip(1)) {
        if prev_id + 2 == next_id {
            println!("{}, {}", prev_id, next_id)
        } 
    }
}

fn calculate_seat_id(seat_string: &str) -> usize {
    let row_id = binary_partition(&seat_string[..7], 'B');
    let column_id = binary_partition(&seat_string[7..], 'R');

    row_id * 8 + column_id
}

fn binary_partition(input: &str, on_char: char) -> usize {
    let binary_vec = input.chars()
    .rev()
    .map(|c| 
        {
            if c == on_char {
                true
            } else {
                false
            }
        }
    );

    binary_vec.enumerate()
        .map(|(i, b)| 2u64.pow(i as u32) * (b as u64 ))
        .fold(0, |acc, val| acc + val) as usize
}