use crate::custom_error::AocError;
use itertools::Itertools;

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

#[derive(Debug, Clone)]
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

fn print_bots(_bots: &[Robot], _width: usize, _height: usize, _iteration: usize) {
    // this is a dumb solution but it did help me find the correct answer manually...
    // uncomment if you want to generate 10k images
    // let mut grid = vec![vec![None; width]; height];
    // bots.iter()
    //     .for_each(|bot| grid[bot.pos.y as usize][bot.pos.x as usize] = Some(0));
    // let mut image = RgbImage::new(width as u32, height as u32);
    // for x in 0..width {
    //     for y in 0..height {
    //         match grid[y][x] {
    //             Some(_) => {
    //                 let pixel = image.get_pixel_mut(x as u32, y as u32);
    //                 *pixel = image::Rgb([255, 255, 255]);
    //             }
    //             None => {
    //                 let pixel = image.get_pixel_mut(x as u32, y as u32);
    //                 *pixel = image::Rgb([0, 0, 0]);
    //             }
    //         }
    //     }
    // }
    // image
    //     .save(format!("day_14/iterations/{iteration}.png"))
    //     .unwrap()
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
    let robots: Vec<_> = lines.map(Robot::parse).collect();
    for i in 0..10000 {
        let mut bots = robots.clone();
        bots.iter_mut().for_each(|robot| {
            robot.simulate(width, height, i);
        });
        print_bots(&bots, width as usize, height as usize, i);
    }
    Ok(String::new())
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
        assert_eq!("", process(input)?);
        Ok(())
    }
}
