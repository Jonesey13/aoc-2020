use std::path::Path;
use std::fs;

fn main() {
    let time_and_buses_string = fs::read_to_string(Path::new("time_and_buses.txt"))
        .expect("Time & buses file not found!");

    let mut time_and_buses_split = time_and_buses_string.split("\n");

    let time = time_and_buses_split.next().unwrap().parse::<usize>().expect("Time not an int!"); 
    let buses_string = time_and_buses_split.next().expect("No Buses String!");

    let buses: Vec<usize> = buses_string.split(",").filter_map(|val| val.parse::<usize>().ok()).collect();

    let mut buses_and_next_times: Vec<(usize, usize)> = buses.iter().map(|b| (*b, b - time % b)).collect();

    buses_and_next_times.sort_by_key(|(_, time)| *time);

    println!("{:?}", buses_and_next_times);

    let (next_bus, next_time) = buses_and_next_times.first().unwrap();

    println!("{} {} {}", next_bus, next_time, next_bus * next_time)
}
