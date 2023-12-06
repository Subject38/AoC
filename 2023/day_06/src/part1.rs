use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let (_, times) = times_str.split_once(':').unwrap();
    let (_, distances) = distances_str.split_once(':').unwrap();
    let times_vec = times
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let distances_vec = distances
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    let cum_sum: u32 = times_vec.iter().zip(distances_vec).map(|(time, distance)| {
        let mut num_ways = 0;
        for i in 0..*time {
            if i * (*time - i) > distance {
                num_ways += 1;
            }
        }
        num_ways
    }).product();

    Ok(format!("{}", cum_sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
