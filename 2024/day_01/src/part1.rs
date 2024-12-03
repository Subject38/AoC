use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in input.lines() {
        // TODO: get rid of ugly unwraps maybe
        let (num1, num2) = line
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        vec1.push(num1);
        vec2.push(num2);
    }
    vec1.sort();
    vec2.sort();
    Ok(format!(
        "{}",
        vec1.iter()
            .zip(vec2.iter())
            .fold(0, |acc, (num1, num2)| acc + num1.abs_diff(*num2))
    ))
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
