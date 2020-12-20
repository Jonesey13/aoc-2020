use std::{collections::HashMap, path::Path};
use std::fs;

fn main() {
    let starting_numbers_string = fs::read_to_string(Path::new("starting_numbers.txt"))
        .expect("Starting numbers file not found!");

    let mut game = Game::new(starting_numbers_string);

    game.run();

    println!("{}", game.get_last_number());
}


struct Game {
    numbers_spoken: HashMap<usize, (usize, usize)>,
    iteration: usize,
    previous_number: usize,
    starting_numbers: Vec<usize>
}

impl Game {
    fn new(starting_numbers_string: String) -> Self {
        Self {
            starting_numbers: starting_numbers_string.split(',').map(|s| s.parse::<usize>().expect("Not a number!")).collect(),
            iteration: 0,
            previous_number: 0,
            numbers_spoken: HashMap::new()
        }
    }

    fn iterate(&mut self) {
        if let Some(number) = self.starting_numbers.get(self.iteration) {
            self.numbers_spoken.insert(*number, (self.iteration, self.iteration));
            self.previous_number = *number;
        } else {
            if let Some((iteration, previous_iteration)) = self.numbers_spoken.get(&self.previous_number) {
                let next_number = iteration - previous_iteration;
                let previous_iteration = self.numbers_spoken.get(&next_number).unwrap_or(&(self.iteration, self.iteration)).0;
                self.numbers_spoken.insert(next_number, (self.iteration, previous_iteration));
                self.previous_number = next_number;
            } else {
                panic!("unreachable!")
            }
        }   

        self.iteration += 1
    }

    fn run(&mut self) {
        while self.iteration < 30000000 {
            self.iterate();
        }
    }

    fn get_last_number(&self) -> usize {
        self.previous_number
    }
}

