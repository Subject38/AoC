use crate::custom_error::AocError;

fn blink(stones: &[usize]) -> Vec<usize> {
    let mut res = Vec::with_capacity(stones.len() * 2);
    for stone in stones {
        let num_digits = if *stone == 0 { 1 } else { stone.ilog10() + 1 };
        if *stone == 0 {
            res.push(1)
        } else if num_digits % 2 == 0 {
            let h1 = stone / (10usize.pow(num_digits / 2));
            let h2 = stone - h1 * 10usize.pow(num_digits / 2);
            res.append(&mut vec![h1, h2])
        } else {
            res.push(stone.saturating_mul(2024))
        }
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    // this does not work on any reasonable computer.
    // at iteration 47 i had 21GB of memory usage so this is clearly the wrong approach...
    for _ in 0..75 {
        stones = blink(&stones);
    }
    Ok(format!("{}", stones.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
