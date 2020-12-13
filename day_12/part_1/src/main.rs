use std::path::Path;
use std::fs;

fn main() {
    let navigation_string = fs::read_to_string(Path::new("navigation.txt"))
        .expect("Navigation file not found!");

    let mut ship = Ship::new(navigation_string);

    ship.run();

    println!("{}", ship.get_final_distance())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize)
}

enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn from_degrees(deg: isize) -> Direction {
        match (deg + 360) % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => { 
                println!("{}", deg);
                panic!("Invalid Degrees for Direction!"); 
            }
        }
    }

    fn to_degrees(&self) -> isize {
        match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270
        }
    }
}

struct Ship {
    instructions: Vec<Instruction>,
    current_direction: Direction,
    current_position: (isize, isize)
}

impl Ship {
    fn new(navigation_string: String) -> Self {
        let instructions = navigation_string.split("\n").map(|line| {
            match (&line[..1], &line[1..]) {
                ("N", val) => Instruction::North(val.parse::<isize>().expect("Not a number!")),
                ("S", val) => Instruction::South(val.parse::<isize>().expect("Not a number!")),
                ("E", val) => Instruction::East(val.parse::<isize>().expect("Not a number!")),
                ("W", val) => Instruction::West(val.parse::<isize>().expect("Not a number!")),
                ("L", val) => Instruction::Left(val.parse::<isize>().expect("Not a number!")),
                ("R", val) => Instruction::Right(val.parse::<isize>().expect("Not a number!")),
                ("F", val) => Instruction::Forward(val.parse::<isize>().expect("Not a number!")),
                _ => panic!("Cannot interpret instruction!")
            }
        }).collect();

        Self {
            instructions,
            current_direction: Direction::East,
            current_position: (0, 0)
        }
    }

    fn interpret_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(val) 
                => self.current_position = (self.current_position.0, self.current_position.1 + val),
            Instruction::South(val) 
                => self.current_position = (self.current_position.0, self.current_position.1 - val),
            Instruction::East(val) 
                => self.current_position = (self.current_position.0 + val, self.current_position.1),
            Instruction::West(val) 
                => self.current_position = (self.current_position.0 - val, self.current_position.1),
            Instruction::Forward(val) => 
            {
                match self.current_direction {
                    Direction::North 
                        => self.current_position = (self.current_position.0, self.current_position.1 + val),
                    Direction::South
                        => self.current_position = (self.current_position.0, self.current_position.1 - val),
                    Direction::East
                        => self.current_position = (self.current_position.0 + val, self.current_position.1),
                    Direction::West
                        => self.current_position = (self.current_position.0 - val, self.current_position.1),
                }
            },
            Instruction::Right(val) => {
                let new_degrees = self.current_direction.to_degrees() + val;

                self.current_direction = Direction::from_degrees(new_degrees);
            },
            Instruction::Left(val) => {
                let new_degrees = self.current_direction.to_degrees() - val;

                self.current_direction = Direction::from_degrees(new_degrees);
            }
        }
    }

    fn run(&mut self) {
        let instructions = self.instructions.clone();
        for instruction in instructions {
            self.interpret_instruction(instruction);
        }
    }

    fn get_final_position(&self) -> (isize, isize) {
        self.current_position
    }

    fn get_final_distance(&self) -> usize {
        let final_position = self.get_final_position();

        final_position.0.abs() as usize + final_position.1.abs() as usize
    }
}