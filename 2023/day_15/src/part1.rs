use crate::custom_error::AocError;

fn hash(value: &str) -> u8 {
    value.chars().fold(0_u8, |hash, c| {
        hash.wrapping_add(c as u8).wrapping_mul(17)            
    })
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let sum = input.lines().next().unwrap().split(',').map(|value| {
        hash(value) as usize
    }).sum::<usize>();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
