use crate::custom_error::AocError;

fn check_line_valid(line: &str) -> bool {
    let (pattern, vals) = line.split_once(' ').expect("Should only be one space");
    let mut springs = pattern.split('.').collect::<Vec<&str>>();
    springs.retain(|&s| s != "");
    let counts = vals.split(',').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    counts.is_empty()
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    for line in input.lines() {
        check_line_valid(line);
    }
    Ok(format!("naww idek man. this problem is fr busted"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
