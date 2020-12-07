use std::collections::{HashMap};
use std::{path::Path};
use std::fs;
use regex::Regex;

fn main() {
    let bags_string = fs::read_to_string(Path::new("bags.txt"))
        .expect("Bags file not found!");

    let bag_strings: Vec<&str> =  bags_string.split("\n").collect();

    let mut bag_counts = HashMap::<String, usize>::new();

    let bags: Vec<BagData> = bag_strings.iter().cloned().map(|s| BagData::new(s)).collect();
    let shiny_gold = bags.iter().find(|b| b.get_bag_name() == "shiny gold").unwrap().clone();
            
    let total_bags = process_bag(&shiny_gold, &bags, &mut bag_counts);

    println!("{}", total_bags - 1 ); // Don't count the shiny bag itself
}

fn process_bag(bag: &BagData, bags: &Vec<BagData>, bag_counts: &mut HashMap::<String, usize>) -> usize {
    if let Some(count) = bag_counts.get(bag.get_bag_name()) {
        return *count;
    } else {
        let mut bag_count = 1;

        for (inner_bag_name, quantity) in bag.get_inner_bags() {
            let inner_bag = bags.iter().find(|b| b.get_bag_name() == inner_bag_name).unwrap();

            let count = process_bag(inner_bag, bags, bag_counts);

            bag_count = bag_count + quantity * count;
        }

        bag_counts.insert(bag.get_bag_name().clone(), bag_count);
        
        return bag_count;
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

    pub fn get_inner_bags(&self) -> &Vec<(String, usize)> {
        &self.inner_bags
    }
}