use crate::custom_error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // cheeky regex because i'm lazy...
    let re = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    Ok(format!(
        "{}",
        re.captures_iter(input)
            .map(|captures| {
                let (_, [val1, val2]) = captures.extract();
                let num1 = val1.parse::<usize>().unwrap();
                let num2 = val2.parse::<usize>().unwrap();
                num1 * num2
            })
            .sum::<usize>()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
