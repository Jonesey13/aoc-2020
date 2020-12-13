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

struct Ship {
    instructions: Vec<Instruction>,
    current_position: (isize, isize),
    waypoint_position: (isize, isize)
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
            current_position: (0, 0),
            waypoint_position: (10, 1)
        }
    }

    fn interpret_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(val) 
                => self.waypoint_position = (self.waypoint_position.0, self.waypoint_position.1 + val),
            Instruction::South(val) 
                => self.waypoint_position = (self.waypoint_position.0, self.waypoint_position.1 - val),
            Instruction::East(val) 
                => self.waypoint_position = (self.waypoint_position.0 + val, self.waypoint_position.1),
            Instruction::West(val) 
                => self.waypoint_position = (self.waypoint_position.0 - val, self.waypoint_position.1),
            Instruction::Forward(val) 
                => self.current_position = (
                    self.current_position.0 + val * self.waypoint_position.0, 
                    self.current_position.1 + val * self.waypoint_position.1
            ),
            Instruction::Right(val) => {
                match val {
                    90 => self.waypoint_position = (self.waypoint_position.1, - self.waypoint_position.0),
                    180 => self.waypoint_position = (-self.waypoint_position.0, -self.waypoint_position.1),
                    270 => self.waypoint_position = (-self.waypoint_position.1, self.waypoint_position.0),
                    _ => panic!("Invalid Angle!")
                }
            },
            Instruction::Left(val) => {
                match val {
                    90 => self.waypoint_position = (-self.waypoint_position.1, self.waypoint_position.0),
                    180 => self.waypoint_position = (-self.waypoint_position.0, -self.waypoint_position.1),
                    270 => self.waypoint_position = (self.waypoint_position.1, -self.waypoint_position.0),
                    _ => panic!("Invalid Angle!")
                }
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