use std::path::Path;
use std::fs;

fn main() {
    let time_and_buses_string = fs::read_to_string(Path::new("time_and_buses.txt"))
        .expect("Time & buses file not found!");

    let mut time_and_buses_split = time_and_buses_string.split("\n");

    let time = time_and_buses_split.next().unwrap().parse::<usize>().expect("Time not an int!"); 
    let buses_string = time_and_buses_split.next().expect("No Buses String!");

    let bus_constraints: Vec<BusConstraint> = buses_string.split(",")
        .enumerate()
        .filter_map(
            |(index, val)| 
                val.parse::<usize>()
                .ok()
                .and_then(|val| Some(BusConstraint::new(index, val)))
        )
        .collect();

    let mut timestamp = time;
    let mut step = 1;

    for constraint in bus_constraints {
        timestamp += step;
        loop {
            if constraint.validate_time(timestamp) {
                step = step * constraint.bus_number;
                break;
            } else {
                timestamp += step;
            }
        }
    }

    println!("{}", timestamp)
}

#[derive(Debug)]
struct BusConstraint {
    time_offset: usize,
    bus_number: usize
}

impl BusConstraint {
    fn new(time_offset: usize, bus_number: usize) -> Self { Self { time_offset, bus_number } }

    fn validate_time(&self, time: usize) -> bool {
        (time + self.time_offset) % self.bus_number == 0
    }
}

