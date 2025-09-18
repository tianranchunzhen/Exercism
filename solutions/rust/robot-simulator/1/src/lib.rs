// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            x: match self.d {
                Direction::North | Direction::South => self.x,
                Direction::West => self.x - 1,
                Direction::East => self.x + 1,
            },
            y: match self.d {
                Direction::West | Direction::East => self.y,
                Direction::South => self.y - 1,
                Direction::North => self.y + 1,
            },
            d: self.d,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut res = self;
        for char in instructions.chars() {
            match char {
                'R' => {
                    res = res.turn_right();
                }
                'L' => {
                    res = res.turn_left();
                }
                'A' => {
                    res = res.advance();
                }
                _ => (),
            }
        }
        res
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
