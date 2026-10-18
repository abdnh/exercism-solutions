#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Direction::North,
            'R' => Direction::East,
            'B' => Direction::South,
            'L' => Direction::West,
            _ => return Err(value),
        })
    }
}

impl TryFrom<i32> for Direction {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => return Err(value),
        })
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn turn(self, change: i32) -> Self {
        let direction: Direction = (((self.direction as i32) + change) % 4).try_into().unwrap();
        let (x, y) = {
            let inc = match (self.direction, direction) {
                (Direction::North, Direction::North) => (0, 1),
                (Direction::East, Direction::East) => (1, 0),
                (Direction::South, Direction::South) => (0, -1),
                (Direction::West, Direction::West) => (-1, 0),
                _ => (0, 0),
            };
            (self.x + inc.0, self.y + inc.1)
        };
        Self { x, y, direction }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        self.turn(1)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        self.turn(3)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        self.turn(0)
    }

    #[must_use]
    pub fn back(self) -> Self {
        self.turn(2)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut new_robot: Robot = self;
        for direction in instructions.chars() {
            let direction: Result<Direction, _> = direction.try_into();
            new_robot = match direction {
                Ok(Direction::North) => new_robot.advance(),
                Ok(Direction::East) => new_robot.turn_right(),
                Ok(Direction::South) => new_robot.back(),
                Ok(Direction::West) => new_robot.turn_left(),
                Err(_) => continue,
            };
        }
        new_robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
