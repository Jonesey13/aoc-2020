use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

fn main() {
    let complete_program_string = fs::read_to_string(Path::new("program.txt"))
        .expect("Program file not found!");

    let programs = complete_program_string.split("mask = ").skip(1).map(|p| Program::new(p.trim()));
    let mut computer = Computer::new();

    for program in programs {
        computer.run_program(&program);
    }

    println!("{}", computer.sum_values());
}

struct Computer {
    values: HashMap<u64, u64>
}

impl Computer {
    fn new() -> Self { 
        Self { 
            values: HashMap::new()
        } 
    }

    fn run_program(&mut self, program: &Program) {
        let mask = program.get_mask();

        for instruction in program.get_instructions() {
            let new_value = mask.apply_to_value(instruction.value);

            self.values.insert(instruction.memory_address, new_value);
        }
    }

    fn sum_values(&self) -> u64 {
        self.values.values().sum()
    }
} 

struct Mask {
    mask: Vec<char>
}

impl Mask {
    fn new(mask_string: &str) -> Self {        
        Self {
            mask: mask_string.chars().collect()
        }
    }

    fn apply_to_value(&self, value: u64) -> u64 {
        let bit_string = format!("{:036b}", value);
        let bit_string_vec: Vec<char> = bit_string.chars().collect();

        let mut output_vec = vec![];

        for (mask_val, bit_val) in self.mask.iter().zip(bit_string_vec.iter()) {
            let next_char = match (mask_val, bit_val) {
                ('1', _) => '1',
                ('0', _) => '0',
                ('X', ch) => *ch,
                (mask_char, val_char) => panic!("Invalid Mask: {:?} {:?}", mask_char, val_char)
            };

            output_vec.push(next_char);
        }

        let output_string: String = output_vec.into_iter().collect();

        u64::from_str_radix(&output_string, 2).expect("Could not parse binary string")
    }
}

struct Program {
    mask: Mask,
    instructions: Vec<Instruction>
}

impl Program {
    fn new(program_string: &str) -> Self {
        let lines: Vec<&str> = program_string.split("\n").collect();
        let mask = Mask::new(lines[0]);

        let instructions = lines[1..].iter().map(|line| Instruction::new(line)).collect();

        Self {
            mask,
            instructions
        }
    }

    fn get_mask(&self) -> &Mask {
        &self.mask
    }

    fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug)]
struct Instruction {
    pub memory_address: u64,
    pub value: u64
}


impl Instruction {
    const REGEX_STRING: &'static str = r"mem\[(\d+)\] = (\d+)";

    fn new(instruction_string: &str) -> Self {
        let regex = Regex::new(Self::REGEX_STRING).expect("Invalid Regex");
        let matches =  regex.captures(instruction_string).expect("Regex match failed for instruction");

        Self {
            memory_address: matches.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            value: matches.get(2).unwrap().as_str().parse::<u64>().unwrap()
        }
    }
}