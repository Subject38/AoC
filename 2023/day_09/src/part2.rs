use crate::custom_error::AocError;

fn get_derivative(series: &[i64]) -> Vec<i64> {
    let mut res = vec![0_i64; series.len() - 1];
    for index in 0..res.len() {
        res[index] = series[index + 1] - series[index]
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let sum: i64 = input
        .lines()
        .map(|line| {
            let series = line
                .split_whitespace()
                .rev()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>();
            let mut derivative = series.clone();
            let mut last_numbers_of_derivatives = vec![0; series.len()];
            let mut count = 0;
            loop {
                derivative = get_derivative(&derivative);
                last_numbers_of_derivatives[count] = *derivative.iter().next_back().unwrap();
                count += 1;
                if derivative.iter().all(|&x| x == 0) {
                    break;
                }
            }
            series.iter().next_back().unwrap() + last_numbers_of_derivatives.iter().sum::<i64>()
        })
        .sum();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
