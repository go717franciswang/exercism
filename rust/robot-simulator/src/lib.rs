// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    d: Direction
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x: x, y: y, d: d }
    }

    pub fn turn_right(self) -> Self {
        Robot::new(self.x, self.y, match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        })
    }

    pub fn turn_left(self) -> Self {
        Robot::new(self.x, self.y, match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        })
    }

    pub fn advance(self) -> Self {
        let (dx, dy) = match self.d {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0)
        };
        Robot::new(self.x + dx, self.y + dy, self.d)
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instr in instructions.chars() {
            robot = match instr {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => panic!()
            }
        }

        robot
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
