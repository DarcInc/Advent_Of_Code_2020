use std::fs::File;
use std::io::{BufRead, BufReader};

enum Command {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Command {
    fn new(text: &str, amt: i32) -> Self {
        match text {
            "N" => Command::North(amt),
            "S" => Command::South(amt),
            "E" => Command::East(amt),
            "W" => Command::West(amt),
            "L" => Command::Left(amt),
            "R" => Command::Right(amt),
            "F" => Command::Forward(amt),
            _ => panic!("Invalid command")
        }
    }

    fn load_command_file(path: &str) -> std::io::Result<Vec<Command>> {
        let input_file = File::open(path)?;
        let reader = BufReader::new(input_file);
        let mut result = vec![];
    
        for line in reader.lines() {
            match line {
                Err(error) => return Err(error),
                Ok(raw_line) => {
                    let (raw_cmd, raw_amt) = raw_line.split_at(1);
                    let amt:i32 = match raw_amt.parse() {
                        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
                        Ok(val) => val
                    };
                    result.push(Command::new(raw_cmd, amt));
                }
            }
        }

        Ok(result)
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn turn(&self, amt: i32) -> Self {
        if amt == 0 {
            *self
        } else if amt > 0 {
            match self {
                Orientation::North => Orientation::East.turn(amt - 90),
                Orientation::South => Orientation::West.turn(amt - 90),
                Orientation::East => Orientation::South.turn(amt - 90),
                Orientation::West => Orientation::North.turn(amt - 90),
            }
        } else {
            match self {
                Orientation::North => Orientation::West.turn(amt + 90),
                Orientation::South => Orientation::East.turn(amt + 90),
                Orientation::East => Orientation::North.turn(amt + 90),
                Orientation::West => Orientation::South.turn(amt + 90),
            }
        }
    }
}

struct Waypoint {
    latitude: i32,
    longitude: i32,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint {
            latitude: 1,
            longitude: 10,
        }
    }

    fn process_command(&mut self, cmd: &Command) {
        match cmd {
            Command::North(amt) => self.latitude += amt,
            Command::South(amt) => self.latitude -= amt,
            Command::East(amt) => self.longitude += amt,
            Command::West(amt) => self.longitude -= amt,
            Command::Left(amt) => {
                match amt {
                    90 | 180 | 270 | 360 => { 
                        let temp = -self.latitude;
                        self.latitude = self.longitude;
                        self.longitude = temp;
                        self.process_command(&Command::Left(amt - 90));
                    },
                    0 => {},
                    _ => panic!("Invalid rotation amount")
                }
            },
            Command::Right(amt) => {
                match amt {
                    90 | 180 | 270 | 360 => {
                        let temp = self.latitude;
                        self.latitude = -self.longitude;
                        self.longitude = temp;
                        self.process_command(&Command::Right(amt - 90));
                    },
                    0 => {},
                    _ => panic!("Invalid rotation amount")
                }
            }
            _ => {}
        }
    }
}

struct Ship {
    latitude: i32,
    longitude: i32,
    orientation: Orientation,
    waypoint: Waypoint,
}

impl Ship {
    fn new() -> Self {
        Ship {
            latitude: 0,
            longitude: 0,
            orientation: Orientation::East,
            waypoint: Waypoint::new(),
        }
    }

    fn process_command(&mut self, cmd: &Command) {
        match cmd {
            Command::North(amt) => self.latitude += amt,
            Command::South(amt) => self.latitude -= amt,
            Command::East(amt) => self.longitude += amt,
            Command::West(amt) => self.longitude -= amt,
            Command::Left(amt) => self.orientation = self.orientation.turn(-amt),
            Command::Right(amt) => self.orientation = self.orientation.turn(*amt),
            Command::Forward(amt) => {
                match self.orientation {
                    Orientation::North => self.latitude += amt,
                    Orientation::South => self.latitude -= amt,
                    Orientation::East => self.longitude += amt,
                    Orientation::West => self.longitude -= amt,
                }
            }
        }
    }

    fn waypoint_navigation(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(amt) => {
                self.latitude += self.waypoint.latitude * amt;
                self.longitude += self.waypoint.longitude * amt;
            },
            _ => self.waypoint.process_command(cmd)
        }
    }
}

fn main() {
    let commands = Command::load_command_file("./input.txt").expect("Failed to load command file");
    let mut ship = Ship::new();
    for cmd in &commands {
        ship.process_command(cmd);
    }
        

    println!("{}, {}", ship.latitude, ship.longitude);
    println!("{}, {} = {}", ship.latitude.abs(), ship.longitude.abs(), ship.latitude.abs() + ship.longitude.abs());

    let mut ship = Ship::new();
    for cmd in &commands {
        ship.waypoint_navigation(cmd);
    }

    println!("{}, {}", ship.latitude, ship.longitude);
    println!("{}, {} = {}", ship.latitude.abs(), ship.longitude.abs(), ship.latitude.abs() + ship.longitude.abs());

}
