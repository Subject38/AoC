use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input.lines().fold(0, |acc, l| {
        let mut line_iter = l.split_whitespace().peekable();
        let mut prev_num: usize = line_iter.next().unwrap().parse().unwrap();
        let increasing = line_iter.peek().unwrap().parse::<usize>().unwrap() > prev_num;
        for item in line_iter {
            let num: usize = item.parse().unwrap();
            if increasing && (num <= prev_num || num - prev_num > 3) {
                return acc;
            }
            if !increasing && (num >= prev_num || prev_num - num > 3) {
                return acc;
            }
            prev_num = num
        }
        acc + 1
    });
    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
