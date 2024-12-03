use itertools::Itertools;
use counter::Counter;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut vector = vec![];
    let mut counter: Counter<usize, usize> = Counter::new();
    for line in input.lines() {
        let (num1, num2) = line.split_whitespace().map(|i| i.parse::<usize>().unwrap()).collect_tuple().unwrap();
        vector.push(num1);
        counter[&num2] += 1;
    }
    vector.sort();
    Ok(format!("{}", vector.iter().fold(0, |acc, num| acc + num * counter[num])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
