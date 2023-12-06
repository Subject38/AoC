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
    let product: u32 = times_vec
        .iter()
        .zip(distances_vec)
        .map(|(time, distance)| {
            (0..*time)
                .map(|i| {
                    if i * (*time - i) > distance {
                        return 1;
                    }
                    0
                })
                .sum::<u32>()
        })
        .product();

    Ok(format!("{}", product))
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
