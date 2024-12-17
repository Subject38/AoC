use std::fmt::Display;

use crate::instruction::Direction;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    pub r: usize,
    pub c: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            r: value.0,
            c: value.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(val: Position) -> (usize, usize) {
        (val.r, val.c)
    }
}

impl Position {
    pub fn move_to(&self, dir: Direction) -> Position {
        use Direction::*;
        match dir {
            North => (self.r.saturating_sub(1), self.c).into(),
            East => (self.r, self.c.saturating_add(1)).into(),
            South => (self.r.saturating_add(1), self.c).into(),
            West => (self.r, self.c.saturating_sub(1)).into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GridObject {
    Wall,
    Robot,
    ObstacleLeft,
    ObstacleRight,
    Empty,
}

#[derive(Debug)]
pub struct Grid {
    inner: Vec<Vec<GridObject>>,
    robot_pos: Position,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GridObject::*;
        let grid = self.inner.iter().fold(String::new(), |a, row| {
            a + &row
                .iter()
                .map(|item| match item {
                    Empty => '.',
                    ObstacleLeft => '[',
                    ObstacleRight => ']',
                    Wall => '#',
                    Robot => '@',
                })
                .collect::<String>()
                + "\n"
        });
        write!(f, "{grid}")
    }
}

impl Grid {
    pub fn parse(grid_str: &str) -> Grid {
        let inner: Vec<Vec<GridObject>> = grid_str
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => GridObject::Wall,
                        '@' => GridObject::Robot,
                        '[' => GridObject::ObstacleLeft,
                        ']' => GridObject::ObstacleRight,
                        '.' => GridObject::Empty,
                        _ => unreachable!("invalid grid"),
                    })
                    .collect()
            })
            .collect();
        let robot_position = inner
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                if let Some((j, _)) = row
                    .iter()
                    .enumerate()
                    .find(|(_, item)| **item == GridObject::Robot)
                {
                    Some((i, j))
                } else {
                    None
                }
            })
            .unwrap();
        Grid {
            inner,
            robot_pos: robot_position.into(),
        }
    }

    // convenience wrappers to save characters. will panic if position is invalid
    fn get(&self, pos: Position) -> &GridObject {
        &self.inner[pos.r][pos.c]
    }

    fn get_mut(&mut self, pos: Position) -> &mut GridObject {
        &mut self.inner[pos.r][pos.c]
    }

    pub fn move_robot(&mut self, dir: Direction) {
        let new_pos = self.robot_pos.move_to(dir);
        match self.get(new_pos) {
            GridObject::Wall => {} // do nothing at all. easiest one.
            GridObject::Empty => {
                // just move the robot into the empty space and replace robot's old position with empty...
                *self.get_mut(self.robot_pos) = GridObject::Empty;
                *self.get_mut(new_pos) = GridObject::Robot;
                self.robot_pos = new_pos;
            }
            GridObject::ObstacleLeft | GridObject::ObstacleRight => {
                // This is the ultimate blocker for this part. Even describing
                // the full behavior and edgecases of this action formally would
                // take forever. This just about demands fragile spaghetti spewing and
                // is a miserable exercise... So I'm leaving the current solution here,
                // for perhaps a smarter and more motivated version of myself to implement...
            }
            GridObject::Robot => unreachable!("Only one robot possible"),
        }
    }

    pub fn solve(&self) -> usize {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                r.iter().enumerate().map(move |(j, c)| {
                    if c == &GridObject::ObstacleLeft {
                        100 * i + j
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}
