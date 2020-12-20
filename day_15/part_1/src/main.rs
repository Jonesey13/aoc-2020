use std::path::Path;
use std::fs;

fn main() {
    let starting_numbers_string = fs::read_to_string(Path::new("starting_numbers.txt"))
        .expect("Starting numbers file not found!");

    let mut game = Game::new(starting_numbers_string);

    game.run();

    println!("{}", game.get_last_number());
}


struct Game {
    numbers_spoken: Vec<usize>,
    iteration: usize,
    starting_numbers: Vec<usize>
}

impl Game {
    fn new(starting_numbers_string: String) -> Self {
        Self {
            starting_numbers: starting_numbers_string.split(',').map(|s| s.parse::<usize>().expect("Not a number!")).collect(),
            iteration: 0,
            numbers_spoken: vec![]
        }
    }

    fn iterate(&mut self) {
        if let Some(number) = self.starting_numbers.get(self.iteration) {
            self.numbers_spoken.push(*number)
        } else {
            let previous_number = self.numbers_spoken[self.iteration - 1];
            if let Some((i, _)) = self.numbers_spoken
                .iter()
                .enumerate()
                .rev()
                .skip(1)
                .find(|(_, num)| **num == previous_number) {
                    self.numbers_spoken.push(self.iteration - 1 - i);
            } else {
                self.numbers_spoken.push(0);
            }
        }

        self.iteration += 1
    }

    fn run(&mut self) {
        while self.iteration < 2020 {
            self.iterate();
        }
    }

    fn get_last_number(&self) -> usize {
        self.numbers_spoken[self.numbers_spoken.len() - 1]
    }
}

