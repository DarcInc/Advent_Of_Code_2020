use std::fs::File;
use std::io::{BufRead, BufReader};

const FLOOR:u8 = '.' as u8;
const OCCUPIED:u8 = '#' as u8;
const FREE:u8 = 'L' as u8;

enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

struct Room {
    locations: Vec<Vec<u8>>,
}

impl Room {
    fn new() -> Self {
        Room {
            locations: vec![],
        }
    }

    fn append_row(&mut self, row: &str) {
        self.locations.push(row.as_bytes().to_vec());
    }

    fn width(&self) -> usize {
        self.locations[0].len()
    }

    fn height(&self) -> usize {
        self.locations.len()
    }

    fn top(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;
        if location.1 > 0 {
            Some(self.locations[y-1][x])
        } else {
            None
        }
    }

    fn top_right(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if y > 0 && x < (self.width() - 1) {
            Some(self.locations[y-1][x+1])
        } else {
            None
        }
    }

    fn right(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if x < self.width() - 1 {
            Some(self.locations[y][x+1])
        } else {
            None
        }
    }

    fn bottom_right(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if x < self.width() - 1 && y < self.height() - 1 {
            Some(self.locations[y+1][x+1])
        } else {
            None
        }
    }

    fn bottom(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if y < self.height() - 1 {
            Some(self.locations[y+1][x])
        } else {
            None
        }
    }

    fn bottom_left(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if y < self.height() - 1 && x > 0 {
            Some(self.locations[y+1][x-1])
        } else {
            None
        }
    }

    fn left(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;
        if x > 0 {
            Some(self.locations[y][x-1])
        } else {
            None
        }
    }

    fn top_left(&self, location: &(usize, usize)) -> Option<u8> {
        let x = location.0;
        let y = location.1;

        if x > 0 && y > 0 {
            Some(self.locations[y-1][x-1])
        } else {
            None
        }
    }

    fn next_location(&self, dir: &Direction, location: &(usize, usize)) -> (usize, usize) {
        match dir {
            Direction::Top => (location.0, location.1 - 1),
            Direction::TopRight => (location.0 + 1, location.1 -1),
            Direction::Right => (location.0 + 1, location.1),
            Direction::BottomRight => (location.0 + 1, location.1 + 1),
            Direction::Bottom => (location.0, location.1 + 1),
            Direction::BottomLeft => (location.0 - 1, location.1 + 1),
            Direction::Left => (location.0 - 1, location.1),
            Direction::TopLeft => (location.0 - 1, location.1 - 1),
        }
    }

    fn visible(&self, dir: &Direction, location: &(usize, usize)) -> Option<u8> {
        let current_next = match dir {
            Direction::Top => self.top(location),
            Direction::TopRight => self.top_right(location),
            Direction::Right => self.right(location),
            Direction::BottomRight => self.bottom_right(location),
            Direction::Bottom => self.bottom(location),
            Direction::BottomLeft => self.bottom_left(location),
            Direction::Left => self.left(location),
            Direction::TopLeft => self.top_left(location),
        };

        match current_next {
            None => current_next,
            Some(square) => {
                if square == FLOOR {
                    self.visible(&dir, &self.next_location(&dir, &location))
                } else {
                    current_next
                }
            }
        }
    }

    fn adjacency(&self, location: &(usize, usize)) -> Vec<Option<u8>> {
        vec![
            self.top(location),
            self.top_right(location),
            self.right(location),
            self.bottom_right(location),
            self.bottom(location),
            self.bottom_left(location),
            self.left(location),
            self.top_left(location),
        ]
    }

    fn visible_adjacency(&self, location: &(usize, usize)) -> Vec<Option<u8>> {
        vec![
            self.visible(&Direction::Top, location),
            self.visible(&Direction::TopRight, location),
            self.visible(&Direction::Right, location),
            self.visible(&Direction::BottomRight, location),
            self.visible(&Direction::Bottom, location),
            self.visible(&Direction::BottomLeft, location),
            self.visible(&Direction::Left, location),
            self.visible(&Direction::TopLeft, location),
        ]
    }

    fn occupancy(&self, location: &(usize, usize)) -> u32 {
        let mut total_occupancy = 0;
        for val in self.adjacency(location) {
            match val {
                None => {},
                Some(square) => {
                    if square == OCCUPIED {
                        total_occupancy += 1;
                    }
                }
            }
        }
        total_occupancy
    }

    fn visible_occupancy(&self, location: &(usize, usize)) -> u32 {
        let mut total_occupancy = 0;
        for val in self.visible_adjacency(location) {
            match val {
                None => {},
                Some(square) => {
                    if square == OCCUPIED {
                        total_occupancy += 1;
                    }
                }
            }
        }
        total_occupancy
    }

    fn next_generation(&self) -> (Self, u32) {
        let mut new_parts:Vec<Vec<u8>> = vec![];
        let mut total_changes = 0;

        for y in 0..self.locations.len() {
            let mut new_row:Vec<u8> = vec![];

            for x in 0..self.locations[y].len() {
                if self.locations[y][x] == FLOOR {
                    new_row.push(FLOOR);
                } else {
                    let adjacency_count = self.occupancy(&(x, y));
                    let current_status = self.locations[y][x];

                    if current_status == FREE && adjacency_count == 0 {
                        total_changes += 1;
                        new_row.push(OCCUPIED);
                    } else if current_status == OCCUPIED && adjacency_count > 3 {
                        new_row.push(FREE);
                        total_changes += 1;
                    } else {
                        new_row.push(current_status);
                    }
                }
            } 

            new_parts.push(new_row);
        }

        (Room {
            locations: new_parts
        }, total_changes)
    }

    fn next_visible_generation(&self) -> (Self, u32) {
        let mut new_parts:Vec<Vec<u8>> = vec![];
        let mut total_changes = 0;

        for y in 0..self.locations.len() {
            let mut new_row:Vec<u8> = vec![];

            for x in 0..self.locations[y].len() {
                if self.locations[y][x] == FLOOR {
                    new_row.push(FLOOR);
                } else {
                    let adjacency_count = self.visible_occupancy(&(x, y));
                    let current_status = self.locations[y][x];

                    if current_status == FREE && adjacency_count == 0 {
                        total_changes += 1;
                        new_row.push(OCCUPIED);
                    } else if current_status == OCCUPIED && adjacency_count > 4 {
                        new_row.push(FREE);
                        total_changes += 1;
                    } else {
                        new_row.push(current_status);
                    }
                }
            } 

            new_parts.push(new_row);
        }

        (Room {
            locations: new_parts
        }, total_changes)
    }

    fn occupied_seats(&self) -> u32 {
        let mut result = 0;

        for row in &self.locations {
            for seat in row {
                if *seat == OCCUPIED {
                    result += 1;
                }
            }
        }

        result
    }
}

fn main() {
    let input_file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(input_file);

    let mut room = Room::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        room.append_row(&line);
    }

    loop {
        let (new_room, changes) = room.next_generation();
        println!("{} changes", changes);
        if changes == 0 {
            println!("Occupied seats: {}", new_room.occupied_seats());
            break;
        }

        room = new_room;
    }

    let input_file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(input_file);

    let mut room = Room::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        room.append_row(&line);
    }

    loop {
        let (new_room, changes) = room.next_visible_generation();
        println!("{} changes", changes);
        if changes == 0 {
            println!("Occupied seats: {}", new_room.occupied_seats());
            break;
        }

        room = new_room;
    }
}
