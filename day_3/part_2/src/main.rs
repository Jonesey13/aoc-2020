use std::path::Path;
use std::fs;

fn main() {
    let slope_string = fs::read_to_string(Path::new("slope.txt"))
        .expect("Slope file not found!");

    let slope = Slope::new(slope_string);

    let travel_directions = vec![
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1)
    ];

    let mut tree_counts = vec![];

    for direction in travel_directions {
        let mut position = (0, 0);
        let mut tree_count = 0;
    
        loop {
            position = (position.0 + direction.0, position.1 + direction.1);
    
            match slope.get_object(position) {
                Some(SlopeObject::Tree) => tree_count += 1,
                None => break,
                _ => continue
            }
        }

        tree_counts.push(tree_count);
    }

    println!("{:?}", tree_counts);

    println!("{}", tree_counts.iter().fold(1u64, |acc, c| acc * c ))
}

struct Slope {
    points: Vec<Vec<SlopeObject>>
}

impl Slope {
    fn new(slope_string: String) -> Self {
        let points = slope_string.split("\n")
            .map(|slope| slope.chars().map(|p| { 
                match p {
                    '#' => SlopeObject::Tree,
                    '.' => SlopeObject::Snow,
                    _ => panic!("Unrecognised Slope Character!")
                }
            }).collect())
        .collect();

        Self {
            points
        }
    }

    fn get_object(&self, position: (usize, usize)) -> Option<SlopeObject> {
        if position.0 < self.points.len() {
            self.points[position.0].clone().into_iter().cycle().nth(position.1)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SlopeObject {
    Tree,
    Snow
}