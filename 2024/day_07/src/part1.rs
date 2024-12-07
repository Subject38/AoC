use itertools::Itertools;

use crate::custom_error::AocError;

fn check_equation(result: usize, operands: &[usize], cur_val: usize) -> bool {
    if operands.is_empty() {
        result == cur_val
    } else {
        check_equation(result, &operands[1..], cur_val + operands[0])
            || check_equation(result, &operands[1..], cur_val * operands[0])
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let equations: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let (result_str, operands_str) = line.split(": ").collect_tuple().unwrap();
            let result = result_str.parse().unwrap();
            let operands = operands_str
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (result, operands)
        })
        .collect();
    let mut ret = 0;
    for (res, operands) in equations {
        ret += if check_equation(res, &operands[1..], operands[0]) {
            res
        } else {
            0
        }
    }
    Ok(format!("{ret}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
