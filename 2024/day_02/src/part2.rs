use crate::custom_error::AocError;
fn valid_line(line: &[i32]) -> bool {
    let mut line_iter = line.iter().peekable();
    let mut prev_num = line_iter.next().unwrap();
    let increasing = line_iter.peek().unwrap() > &prev_num;
    for item in line_iter {
        let num = item;
        if increasing && (num <= prev_num || num - prev_num > 3) {
            return false;
        }
        if !increasing && (num >= prev_num || prev_num - num > 3) {
            return false;
        }
        prev_num = num
    }
    true
}
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input.lines().fold(0, |acc, l| {
        let line_vec = l
            .split_ascii_whitespace()
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if valid_line(&line_vec) {
            return acc + 1;
        }
        for i in 0..line_vec.len() {
            // todo: optimize??? lmfao
            if valid_line(&[&line_vec[0..i], &line_vec[i + 1..]].concat()) {
                return acc + 1;
            }
        }
        acc
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
1 3 6 7 9
1 3 2 3 5
1 2 8 4 5";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
