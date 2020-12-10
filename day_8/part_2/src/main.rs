use std::{collections::HashSet, path::Path};
use std::fs;

fn main() {
    let instructions_string = fs::read_to_string(Path::new("instructions.txt"))
        .expect("Instructions file not found!");

    let mut console = HandheldConsole::new(instructions_string);

    let length_of_program = console.get_instructions().len();

    for i in 0..length_of_program {
        let current_instruction = console.get_instruction(i).clone();

        let new_instruction = match current_instruction {
            ConsoleInstruction::Nop(val) => ConsoleInstruction::Jmp(val),
            ConsoleInstruction::Jmp(val) => ConsoleInstruction::Nop(val),
            _ => continue
        };

        console.set_instruction(i, new_instruction);

        let (acc, pos) = console.run();

        if pos == length_of_program {
            println!("{} {}", acc, i);
            return;
        }

        console.set_instruction(i, current_instruction);

        console.reset()
    }
}

struct HandheldConsole {
    acc: isize,
    current_address: usize,
    instructions: Vec<ConsoleInstruction>,
    instructions_visited: HashSet<usize>
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ConsoleInstruction {
    Nop(isize),
    Jmp(isize),
    Acc(isize)
}

impl HandheldConsole {
    pub fn new(instructions_string: String) -> Self {
        let instructions =  instructions_string.split("\n");
        let instruction_and_values = instructions.map(|instr| instr.split(" "));

        let instructions = instruction_and_values.map(|mut instr| {
            match instr.next().expect("No Instruction") {
                "nop" => ConsoleInstruction::Nop(instr.next().expect("No Value").parse::<isize>().expect("Value isn't a number")),
                "jmp" => ConsoleInstruction::Jmp(instr.next().expect("No Value").parse::<isize>().expect("Value isn't a number")),
                "acc"=> ConsoleInstruction::Acc(instr.next().expect("No Value").parse::<isize>().expect("Value isn't a number")),
                _ => panic!("Invalid Instruction")
            }
        }
        ).collect();

        Self {
            acc: 0,
            current_address: 0,
            instructions,
            instructions_visited: HashSet::new()
        }
    }

    pub fn run (&mut self) -> (isize, usize) {
        loop {
            if self.process_instruction() {
                break;
            }
        }

        (self.get_acc(), self.current_address)
    }

    fn process_instruction(&mut self) -> bool {
        if self.current_address == self.instructions.len() {
            return true;
        }

        let current_instruction = self.instructions[self.current_address];

        if self.instructions_visited.get(&self.current_address).is_some() {
            true
        } else {
            self.instructions_visited.insert(self.current_address);

            match current_instruction {
                ConsoleInstruction::Nop(_) => self.current_address +=1,
                ConsoleInstruction::Acc(val) => { self.acc += val; self.current_address += 1},
                ConsoleInstruction::Jmp(val) if val >= 0 => self.current_address += val as usize,
                ConsoleInstruction::Jmp(val) if val < 0 => self.current_address -= -val as usize,
                _ => panic!("Unreachable!")
            };

            false
        }
    }

    pub fn get_acc(&self) -> isize {
        self.acc
    }

    pub fn get_instructions(&self) -> &Vec<ConsoleInstruction> {
        &self.instructions
    }

    pub fn get_instruction(&self, i: usize) -> &ConsoleInstruction {
        &self.instructions[i]
    }

    pub fn set_instruction(&mut self, i: usize, instr: ConsoleInstruction) {
        self.instructions[i] = instr;
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.current_address = 0;
        self.instructions_visited = HashSet::new();
    }
}