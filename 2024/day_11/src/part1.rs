use crate::custom_error::AocError;
use counter::Counter;

fn blink(stones: &Counter<usize>) -> Counter<usize> {
    let mut res = Counter::new();
    for (stone, count) in stones {
        let num_digits = if *stone == 0 { 1 } else { stone.ilog10() + 1 };
        if *stone == 0 {
            res[&1] += count;
        } else if num_digits % 2 == 0 {
            let h1 = stone / (10usize.pow(num_digits / 2));
            let h2 = stone - h1 * 10usize.pow(num_digits / 2);
            res[&h1] += count;
            res[&h2] += count;
        } else {
            res[&(stone * 2024)] += count;
        }
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut stones: Counter<_> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    for _ in 0..25 {
        stones = blink(&stones);
    }
    Ok(format!("{}", stones.values().sum::<usize>()))
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
