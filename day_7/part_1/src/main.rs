use std::collections::{HashMap};
use std::{path::Path};
use std::fs;
use regex::Regex;

fn main() {
    let bags_string = fs::read_to_string(Path::new("bags.txt"))
        .expect("Bags file not found!");

    let bag_strings: Vec<&str> =  bags_string.split("\n").collect();

    let mut bag_statuses = HashMap::<String, bool>::new();

    let bags: Vec<BagData> = bag_strings.iter().cloned().map(|s| BagData::new(s)).collect();
    
    for bag in &bags {
        process_bag(&bag, &bags, &mut bag_statuses);
    }

    println!("{}", bag_statuses.values().filter(|value| **value).count());
}

fn process_bag(bag: &BagData, bags: &Vec<BagData>, bag_statuses: &mut HashMap::<String, bool>) {
    if bag_statuses.get(bag.get_bag_name()).is_some() {
        return
    } else {
        for inner_bag_name in bag.get_inner_bag_names() {
            if inner_bag_name == "shiny gold" {
                bag_statuses.insert(bag.get_bag_name().clone(), true); 
                return;
            }

            let inner_bag = bags.iter().find(|b| b.get_bag_name() == inner_bag_name).unwrap();

            process_bag(inner_bag, bags, bag_statuses);
            match bag_statuses.get(inner_bag_name) {
                Some(true) => {bag_statuses.insert(bag.get_bag_name().clone(), true); return;},
                _ => ()
            }
        }
        bag_statuses.insert(bag.get_bag_name().clone(), false);
    }
}

#[derive(Clone)]
struct BagData {
    bag_name: String,
    inner_bags: Vec<(String, usize)>
}

impl BagData {
    const INNER_BAG_REGEX_STRING: &'static str = r"(\d+) (.*) bag";

    pub fn new(line_string: &str) -> Self {
        let mut bag_to_inner_bags = line_string.split(" contain ");

        let bag_name = bag_to_inner_bags.next().unwrap().split(" bags").nth(0).unwrap().to_string();
        let inner_bags_string= match bag_to_inner_bags.next() {
            Some(m) => m,
            _ => panic!("No next term for {}", line_string)
        };

        let inner_bags_strings = inner_bags_string.split(",");

        let mut inner_bags: Vec<(String, usize)> = vec![];
        let inner_bag_regex = Regex::new(Self::INNER_BAG_REGEX_STRING).expect("Invalid Regex");

        for bag_string in inner_bags_strings {
            if bag_string == "no other bags." {
                continue
            }

            let matches =  match inner_bag_regex.captures(bag_string) {
                Some(m) => m,
                _ => panic!("No match for {}", bag_string)
            };

            inner_bags.push((matches.get(2).unwrap().as_str().into(), matches.get(1).unwrap().as_str().parse::<usize>().unwrap()));
        }

        Self {
            bag_name,
            inner_bags
        }
    }

    pub fn get_bag_name(&self) -> &String {
        &self.bag_name
    }

    pub fn get_inner_bag_names(&self) -> Vec<&String> {
        self.inner_bags.iter().map(|inner| &inner.0).collect()
    }
}