use crate::custom_error::AocError;

const BUTTON_A: isize = 3;
const BUTTON_B: isize = 1;

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl Machine {
    fn parse(machine_str: &str) -> Machine {
        let mut machine_lines = machine_str.lines();
        let mut bt_a = machine_lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ");
        let mut bt_b = machine_lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ");
        let mut prz = machine_lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ");
        let button_a = Button {
            x: bt_a
                .next()
                .unwrap()
                .split_once('+')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
            y: bt_a
                .next()
                .unwrap()
                .split_once('+')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
        };
        let button_b = Button {
            x: bt_b
                .next()
                .unwrap()
                .split_once('+')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
            y: bt_b
                .next()
                .unwrap()
                .split_once('+')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
        };
        let prize = Prize {
            x: prz
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
            y: prz
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse::<isize>()
                .unwrap(),
        };
        Machine {
            a: button_a,
            b: button_b,
            prize,
        }
    }

    fn solve(self) -> Option<isize> {
        let (ax, ay, bx, by, px, py) = (
            self.a.x,
            self.a.y,
            self.b.x,
            self.b.y,
            self.prize.x,
            self.prize.y,
        );
        let den_b = bx * ay - by * ax;
        let num_b = px * ay - py * ax;
        if num_b % den_b == 0 {
            let b = num_b / den_b;
            let num_a = px - b * bx;
            let den_a = ax;
            if num_a % den_a == 0 {
                let a = (px - b * bx) / ax;
                return Some(a * BUTTON_A + b * BUTTON_B);
            }
        }
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(format!(
        "{}",
        input
            .split("\n\n")
            .map(Machine::parse)
            .filter_map(Machine::solve)
            .sum::<isize>()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
