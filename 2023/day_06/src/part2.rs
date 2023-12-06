use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (times_str, distances_str) = input.split_once('\n').unwrap();
    let (_, times) = times_str.split_once(':').unwrap();
    let (_, distances) = distances_str.split_once(':').unwrap();
    let mut time_s = times.to_owned();
    time_s.retain(|c| !c.is_whitespace());
    let mut distance_s = distances.to_owned();
    distance_s.retain(|c| !c.is_whitespace());
        
    let time: u64 = time_s.parse().unwrap();
    let distance: u64 = distance_s.parse().unwrap();
    let first_time = (0..time).find(|i| {
        i * (time - i) > distance
    }).unwrap();
    let last_time = (0..time).rev().find(|i| {
        i * (time - i) > distance
    }).unwrap();
    Ok(format!("{}", last_time - first_time + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
