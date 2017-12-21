use util::files::read_file_to_string;

use std::fmt;
use std::ops::Add;

pub fn main() {
    println!("-- Running Day 11 --");
    let input = &read_file_to_string("src/days/input/day_11.txt");
    
    let directions = parse_directions(input);
    println!("Direction count: {}", directions.len());

    let origin = HexCoords {x: 0, y: 0};

    let (final_pos, furthest_pos) = execute_directions(origin, directions);
    println!("Final position: {}", final_pos);
    println!("Moves to reach final position from {} is {}", origin, final_pos.moves_to_reach_from(origin));

    println!("Furthest position: {}", furthest_pos);
    println!("Moves to reach furthest position from {} is {}", origin, furthest_pos.moves_to_reach_from(origin));
}

#[derive(Copy, Clone)]
struct HexCoords {
    x: isize,
    y: isize
}

#[derive(Copy, Clone)]
enum HexCardinal { //North = +x axis
    North,
    NorthEast,
    NorthWest,
    South,
    SouthWest,
    SouthEast
}

impl HexCoords {
    fn z(&self) -> isize {
        -(self.x + self.y)
    }

    fn moves_to_reach_from(&self, start: HexCoords) -> usize {
        let numerator = (self.x - start.x).abs() + (self.y - start.y).abs() + (self.z() - start.z()).abs();
        numerator as usize / 2
    }
}

impl fmt::Display for HexCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Add for HexCoords {
    type Output = HexCoords;

    fn add(self, other: HexCoords) -> HexCoords {
        HexCoords {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl HexCardinal {
    fn hex_vector(self) -> HexCoords {
        match self { //North is to the right on our guide. http://devmag.org.za/2013/08/31/geometry-with-hex-coordinates/
            HexCardinal::North => HexCoords {x: 1, y: 0},
            HexCardinal::South => HexCoords {x: -1, y: 0},
            HexCardinal::NorthEast => HexCoords {x: 1, y: -1},
            HexCardinal::NorthWest => HexCoords {x: 0, y: 1},
            HexCardinal::SouthEast => HexCoords {x: 0, y: -1},
            HexCardinal::SouthWest => HexCoords {x: -1, y: 1},
        }
    }

    fn from(string_rep: &str) -> HexCardinal {
        match string_rep {
            "n" => HexCardinal::North,
            "s" => HexCardinal::South,
            "ne" => HexCardinal::NorthEast,
            "nw" => HexCardinal::NorthWest,
            "se" => HexCardinal::SouthEast,
            "sw" => HexCardinal::SouthWest,
            _ => {
                panic!("Unrecognized HexCardinal {}", string_rep);
            }
        }
    }
}

fn parse_directions(input: &str) -> Vec<HexCardinal> {
    input.split(",").map(|direction| {
        HexCardinal::from(direction)
    }).collect()
}

fn execute_directions(start: HexCoords, directions: Vec<HexCardinal>) -> (HexCoords, HexCoords) {
    let mut final_coords = start;
    let mut furthest_coords = start;

    for step in directions {
        final_coords = final_coords + step.hex_vector();

        if final_coords.moves_to_reach_from(start) > furthest_coords.moves_to_reach_from(start) {
            furthest_coords = final_coords;
        }
    }

    (final_coords, furthest_coords)
}