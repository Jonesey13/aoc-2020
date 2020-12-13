use std::{collections::HashMap, ops::Add, path::Path};
use std::fs;

fn main() {
    let seating_string = fs::read_to_string(Path::new("seating_plan.txt"))
        .expect("Seating file not found!");

    let mut plane = Plane::new(seating_string);

    plane.run();

    println!("{}", plane.build_seats_string());
    
    println!("{}", plane.count_occupied());
}


struct Plane {
    seats: HashMap<PlanePosition, PlaneSquare>,
    bounds: (usize, usize)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PlaneSquare {
    EmptySeat,
    OccupiedSeat
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
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

        let row_count = seating_string.split("\n").count();
        let column_count = seating_string.split("\n").nth(0).unwrap().chars().count();

        Self {
            seats,
            bounds: (row_count, column_count)
        }
    }

    fn find_adjacent_seats(&self, pos: &PlanePosition) -> Vec<PlanePosition> {
        let search_directions: Vec<PlanePosition> = vec![
            PlanePosition::new( 1 as isize, 0 as isize),
            PlanePosition::new(- 1 as isize,  0 as isize),
            PlanePosition::new( 0 as isize,  1 as isize),
            PlanePosition::new( 0 as isize, - 1 as isize),
            PlanePosition::new( 1 as isize,  1 as isize),
            PlanePosition::new(- 1 as isize, - 1 as isize),
            PlanePosition::new( 1 as isize, - 1 as isize),
            PlanePosition::new(- 1 as isize,  1 as isize)
        ];

        let mut output = vec![];

        for direction in search_directions {
            let mut search_pos = *pos;

            search_pos = PlanePosition::new(search_pos.row + direction.row, search_pos.col + direction.col);
            
            while self.is_position_within_bounds(&search_pos) {
                if let Some(_) = self.seats.get(&search_pos) {
                    output.push(search_pos);
                    break
                }
                search_pos = PlanePosition::new(search_pos.row + direction.row, search_pos.col + direction.col);
            }
        }

        output
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
            (PlaneSquare::OccupiedSeat, count) if count >= 5 => PlaneSquare::EmptySeat,
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

    fn is_position_within_bounds(&self, pos: &PlanePosition) -> bool {
        if pos.row >= 0 && pos.row < self.bounds.0 as isize && pos.col >= 0 && pos.col < self.bounds.1 as isize {
            true
        } else {
            false
        }
    }

    fn build_seats_string(&self) -> String {
        let mut seat_string = String::new();

        for x in 0..self.bounds.0 {
            for y in 0..self.bounds.1 {
                let next_char = match self.seats.get(&PlanePosition::new(x as isize, y as isize)) {
                    Some(PlaneSquare::EmptySeat) => "L",
                    Some(PlaneSquare::OccupiedSeat) => "#",
                    _ => "."
                };
                seat_string = seat_string.add(next_char);
            }
            seat_string = seat_string.add("\n");
        }

        seat_string
    }
}