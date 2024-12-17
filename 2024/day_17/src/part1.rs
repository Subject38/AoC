use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Clone, Copy, Debug)]
enum Combo {
    Zero,
    One,
    Two,
    Three,
    A,
    B,
    C,
}

impl Combo {
    pub fn new(op: &str) -> Self {
        match op {
            "0" => Self::Zero,
            "1" => Self::One,
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::A,
            "5" => Self::B,
            "6" => Self::C,
            "7" => panic!("Reserved operand. invalid program!"),
            _ => unreachable!("invalid operand"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Adv(Combo),
    Bxl(usize),
    Bst(Combo),
    Jnz(usize),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    use Combo::*;
    use Operation::*;
    let (regs_str, prog_str) = input.split_once("\n\n").unwrap();
    let (mut reg_a, mut reg_b, mut reg_c) = regs_str
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let prog: Vec<Operation> = prog_str
        .trim_end()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            // let operand = Combo::new(operand);
            let (opcode, operand) = chunk.collect_tuple().unwrap();
            match opcode.parse::<usize>().unwrap() {
                0 => Adv(Combo::new(operand)),
                1 => Bxl(operand.parse::<usize>().unwrap()),
                2 => Bst(Combo::new(operand)),
                3 => Jnz(operand.parse::<usize>().unwrap()),
                4 => Bxc,
                5 => Out(Combo::new(operand)),
                6 => Bdv(Combo::new(operand)),
                7 => Cdv(Combo::new(operand)),
                _ => unreachable!("invalid program"),
            }
        })
        .collect();
    println!("{:?}", prog);
    let mut ip = 0; // default instruction pointer
    let mut res = vec![];
    while ip != prog.len() {
        let operation = prog[ip];
        match operation {
            Adv(operand) => {
                let num = reg_a;
                let den = match operand {
                    Zero => 0,
                    One => 2,
                    Two => 4,
                    Three => 8,
                    A => 2usize.pow(reg_a as u32),
                    B => 2usize.pow(reg_b as u32),
                    C => 2usize.pow(reg_c as u32),
                };
                reg_a = num / den;
                ip += 1;
            }
            Bxl(operand) => {
                reg_b ^= operand;
                ip += 1;
            }
            Bst(operand) => {
                reg_b = match operand {
                    Zero => 0,
                    One => 1,
                    Two => 2,
                    Three => 3,
                    A => reg_a % 8,
                    B => reg_b % 8,
                    C => reg_c % 8,
                };
                ip += 1
            }
            Jnz(operand) => {
                if reg_a != 0 {
                    ip = operand / 2; // thankfully operand is always even...
                } else {
                    ip += 1
                }
            }
            Bxc => {
                reg_b ^= reg_c;
                ip += 1;
            }
            Out(operand) => {
                let output = match operand {
                    Zero => 0,
                    One => 1,
                    Two => 2,
                    Three => 3,
                    A => reg_a % 8,
                    B => reg_b % 8,
                    C => reg_c % 8,
                };
                res.push(output);
                ip += 1;
            }
            Bdv(operand) => {
                let den = match operand {
                    Zero => 0,
                    One => 2,
                    Two => 4,
                    Three => 8,
                    A => 2usize.pow(reg_a as u32),
                    B => 2usize.pow(reg_b as u32),
                    C => 2usize.pow(reg_c as u32),
                };
                reg_b = reg_a / den;
                ip += 1;
            }
            Cdv(operand) => {
                let den = match operand {
                    Zero => 0,
                    One => 2,
                    Two => 4,
                    Three => 8,
                    A => 2usize.pow(reg_a as u32),
                    B => 2usize.pow(reg_b as u32),
                    C => 2usize.pow(reg_c as u32),
                };
                reg_c = reg_a / den;
                ip += 1;
            }
        }
    }
    Ok(res.iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
