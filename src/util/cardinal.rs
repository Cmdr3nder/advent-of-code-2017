use std::fmt;

pub enum Cardinal {
    North,
    South,
    East,
    West
}

impl Cardinal {
    pub fn turn_left(&self) -> Cardinal {
        match *self {
            Cardinal::North => Cardinal::West,
            Cardinal::South => Cardinal::East,
            Cardinal::East  => Cardinal::North,
            Cardinal::West  => Cardinal::South
        }
    }

    pub fn calc_offset(&self, current_coords: (i32, i32)) -> (i32, i32) {
        let offset: (i32, i32) = match *self {
            Cardinal::North => (0, 1),
            Cardinal::South => (0, -1),
            Cardinal::East => (1, 0),
            Cardinal::West => (-1, 0)
        };
        offset 
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Cardinal::North => "North",
            Cardinal::South => "South",
            Cardinal::East => "East",
            Cardinal::West => "West"
        })
    }
}