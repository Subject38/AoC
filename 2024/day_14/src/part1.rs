use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
struct Velocity {
    vx: isize,
    vy: isize,
}
impl From<(isize, isize)> for Velocity {
    fn from(value: (isize, isize)) -> Self {
        Self {
            vx: value.0,
            vy: value.1,
        }
    }
}

#[derive(Debug)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Robot {
    pub fn parse(robot_str: &str) -> Robot {
        let (pos_str, vel_str) = robot_str.split_once(' ').unwrap();
        let (pos_x, pos_y) = pos_str
            .split_once('=')
            .unwrap()
            .1
            .split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vel_x, vel_y) = vel_str
            .split_once('=')
            .unwrap()
            .1
            .split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        Robot {
            pos: (pos_x, pos_y).into(),
            vel: (vel_x, vel_y).into(),
        }
    }

    pub fn simulate(&mut self, width: isize, height: isize, seconds: usize) {
        for _ in 0..seconds {
            self.pos.x = (self.pos.x + self.vel.vx) % width;
            if self.pos.x < 0 {
                self.pos.x += width;
            }
            self.pos.y = (self.pos.y + self.vel.vy) % height;
            if self.pos.y < 0 {
                self.pos.y += height;
            }
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    // super cringe hack to deal with different size for test case vs real input...
    let (width, height) = if input.chars().next().unwrap().is_ascii_digit() {
        lines
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap()
    } else {
        (101, 103)
    };
    let mut robots: Vec<_> = lines.map(Robot::parse).collect();
    let mut quadrant_totals = [0; 4];
    robots.iter_mut().for_each(|robot| {
        robot.simulate(width, height, 100);
        if robot.pos.x != width / 2 && robot.pos.y != height / 2 {
            let quadrant = if robot.pos.x < width / 2 && robot.pos.y < height / 2 {
                0
            } else if robot.pos.x > width / 2 && robot.pos.y < height / 2 {
                1
            } else if robot.pos.x > width / 2 && robot.pos.y > height / 2 {
                2
            } else {
                3
            };
            quadrant_totals[quadrant] += 1;
        }
    });
    Ok(format!("{}", quadrant_totals.iter().product::<i32>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11 7
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
