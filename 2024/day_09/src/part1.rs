use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut disk: Vec<Option<usize>> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .flat_map(|(index, c)| {
            let size: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if index % 2 == 0 {
                vec![Some(index / 2); size]
            } else {
                vec![None; size]
            }
        })
        .collect();
    let (mut p1, mut p2) = (0usize, disk.len() - 1);
    while p1 != p2 && p1 < p2 {
        while disk[p1].is_some() {
            p1 += 1
        }
        while disk[p2].is_none() {
            p2 -= 1
        }
        (disk[p1], disk[p2]) = (disk[p2], disk[p1]);
    }
    (disk[p1], disk[p2]) = (disk[p2], disk[p1]);

    Ok(format!(
        "{}",
        disk.iter()
            .enumerate()
            .fold(0, |acc, (id, val)| acc + id * val.unwrap_or(0))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
