use std::{fmt::Debug, fs::File};
use std::io::{BufReader, prelude::*};

/// Identifies the contents of the square
#[derive(Copy, Clone, PartialEq)]
enum Square {
    Open = 0,
    Tree = 10,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Tree => f.write_str("#"),
            _ => f.write_str("."),
        }
    }
}

trait MovePolicy {
    fn next_move(&mut self, map: &Map, location: (i32, i32)) -> Result<(i32, i32), &'static str>;
}

struct TreeCountingMovePolicy {
    tree_count: i32,
    across: i32,
    down: i32,
}

impl TreeCountingMovePolicy {
    fn new(across: i32, down: i32) -> Self {
        TreeCountingMovePolicy { 
            tree_count: 0,
            across,
            down,
        }
    }
}

impl MovePolicy for TreeCountingMovePolicy {
    fn next_move(&mut self, map: &Map, location: (i32, i32)) -> Result<(i32, i32),&'static str> {
        let new_location = map.move_with_wrap(location, self.across, self.down)?;
        if map.sense_location(new_location)? == Square::Tree {
            self.tree_count += 1;
        }
        Ok(new_location)
    }
}


struct Map {
    data: Vec<Vec<Square>>,
}

impl Map {
    fn new() -> Self {
        Map {
            data: vec![],
        }
    }

    fn append_row(&mut self, row: &str) {
        let mut new_row: Vec<Square> = vec![];
        for char in row.chars() {
            match char {
                '#' => new_row.push(Square::Tree),
                _ => new_row.push(Square::Open),
            }
        }

        self.data.push(new_row);
    }

    fn bounds(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    fn move_with_wrap(&self, location: (i32, i32), across: i32, down: i32) -> Result<(i32, i32), &'static str> {
        if across > (self.data[0].len() as i32) || (down > self.data.len() as i32) {
            return Err("Out of bounds");
        }

        let new_horizontal = if location.0 + across >= (self.data[0].len() as i32) {
            location.0 + across - (self.data[0].len() as i32)
        } else if location.0 + across < 0 {
            location.0 + across + (self.data[0].len() as i32)
        } else {
            location.0 + across
        };

        let new_vertical = if location.1 + down >= (self.data.len() as i32) {
            location.1 + down - (self.data.len() as i32)
        } else if location.1 + down < 0 {
            location.1 + down + (self.data.len() as i32)
        } else {
            location.1 + down
        };

        Ok((new_horizontal, new_vertical))
    }

    fn sense_location(&self, location: (i32, i32)) -> Result<Square, &'static str> {
        if location.0 > (self.data[0].len() as i32) || location.1 > (self.data.len() as i32) {
            Err("Out of bounds")
        } else {
            Ok(self.data[location.1 as usize][location.0 as usize])
        }
    }
}


fn main() {
    let mut tobogan_map = Map::new();

    match File::open("./input.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(the_line) = line {
                    tobogan_map.append_row(&the_line);
                }
            }
        },
        Err(error) => println!("Failed to load data: {}", error),
    }


    let policies: Vec<TreeCountingMovePolicy> = vec![
        TreeCountingMovePolicy::new(1, 1),
        TreeCountingMovePolicy::new(3, 1),
        TreeCountingMovePolicy::new(5, 1),
        TreeCountingMovePolicy::new(7, 1),
        TreeCountingMovePolicy::new(1, 2),
    ];

    let mut trees_hit: Vec<i32> = vec![];
    for mut policy in policies {
        let mut current_location = (0, 0);
        let mut idx = 0;
        while idx < tobogan_map.bounds().1 {
            current_location = policy.next_move(&tobogan_map, current_location).unwrap();
            idx += policy.down as usize;
        }

        trees_hit.push(policy.tree_count);
        println!("You hit {} trees with policy {},{}", policy.tree_count, policy.across, policy.down);
    }

    let mut prod = trees_hit[0] as i64;
    for i in 1..trees_hit.len() {
        prod *= trees_hit[i] as i64;
    }

    println!("The product is: {}", prod);
}
