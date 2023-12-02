use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut sum = 0;
    for line in input.split('\n') {
        let mut characters = line.chars().filter(char::is_ascii_digit);
        let first_digit = characters.next().map_or(0, |c| c.to_digit(10).unwrap());
        let last_digit = characters
            .next_back()
            .map_or(first_digit, |c| c.to_digit(10).unwrap());
        sum += first_digit * 10 + last_digit;
    }
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
