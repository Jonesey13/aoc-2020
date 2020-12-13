use std::{collections::HashMap, path::Path};
use std::fs;

fn main() {
    let seating_string = fs::read_to_string(Path::new("seating_plan.txt"))
        .expect("Seating file not found!");

    let mut plane = Plane::new(seating_string);

    plane.run();

    println!("{}", plane.count_occupied())
}


struct Plane {
    seats: HashMap<PlanePosition, PlaneSquare>
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PlaneSquare {
    EmptySeat,
    OccupiedSeat
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct PlanePosition {
    row: isize,
    col: isize
}

impl PlanePosition {
    fn new(row: isize, col: isize) -> Self {
        Self {
            row,
            col
        }
    }
}

impl Plane {
    fn new(seating_string: String) -> Self {
        let mut seats = HashMap::new();

        for (row, row_string) in seating_string.split("\n").enumerate() {
            for (col, char) in row_string.chars().enumerate() {
                match char {
                    'L' => { seats.insert(PlanePosition::new(row as isize, col as isize), PlaneSquare::EmptySeat); },
                    '#' => { seats.insert(PlanePosition::new(row as isize, col as isize), PlaneSquare::OccupiedSeat); },
                    _ => ()
                }
            }
        }

        Self {
            seats
        }
    }

    fn find_adjacent_seats(&self, pos: &PlanePosition) -> Vec<PlanePosition> {
        vec![
            PlanePosition::new(pos.row + 1, pos.col + 0),
            PlanePosition::new(pos.row - 1, pos.col + 0),
            PlanePosition::new(pos.row + 0, pos.col + 1),
            PlanePosition::new(pos.row + 0, pos.col - 1),
            PlanePosition::new(pos.row + 1, pos.col + 1),
            PlanePosition::new(pos.row - 1, pos.col - 1),
            PlanePosition::new(pos.row + 1, pos.col - 1),
            PlanePosition::new(pos.row - 1, pos.col + 1)
        ]
    }

    fn count_adjacent_occupied_seats(&self, pos: &PlanePosition) -> isize {
        let mut count = 0;

        for adj_pos in self.find_adjacent_seats(pos) {
            match self.seats.get(&adj_pos) {
                Some(PlaneSquare::OccupiedSeat) => count += 1,
                _ => ()
            }
        }

        count
    }

    fn generate_seat_status(&self, pos: &PlanePosition, status: &PlaneSquare) -> PlaneSquare {
        match (status, self.count_adjacent_occupied_seats(pos)) {
            (PlaneSquare::EmptySeat, 0) => PlaneSquare::OccupiedSeat,
            (PlaneSquare::OccupiedSeat, count) if count >= 4 => PlaneSquare::EmptySeat,
            _ => *status 
        }
    }

    fn iterate(&mut self) -> bool {
        let mut new_seats = HashMap::new();

        for (pos, status) in &self.seats {
            let new_status = self.generate_seat_status(pos, status);

            new_seats.insert(*pos, new_status);
        }

        if self.seats == new_seats {
            true
        } else {
            self.seats = new_seats;
            false
        }
    }

    fn run(&mut self) {
        while !self.iterate() {}
    }

    fn count_occupied(&self) -> usize {
        self.seats.values().filter(|square| **square == PlaneSquare::OccupiedSeat).count()
    }
}