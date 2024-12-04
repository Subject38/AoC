use crate::custom_error::AocError;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // cheeky regex because i'm lazy...
    let re =
        Regex::new(r"mul\((?<nums>\d{1,3},\d{1,3})\)|(?<do>do\(\))|(?<hiiii>don't\(\))").unwrap();
    let mut enabled = true;
    Ok(format!(
        "{}",
        re.captures_iter(input)
            .map(|captures| {
                let (_, [val]) = captures.extract();
                match val {
                    "do()" => {
                        enabled = true;
                        0
                    }
                    "don\'t()" => {
                        enabled = false;
                        0
                    }
                    _ => {
                        if enabled {
                            let (val1, val2) = val.split_once(',').unwrap();
                            let (num1, num2) = (
                                val1.parse::<usize>().unwrap(),
                                val2.parse::<usize>().unwrap(),
                            );
                            num1 * num2
                        } else {
                            0
                        }
                    }
                }
            })
            .sum::<usize>()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
