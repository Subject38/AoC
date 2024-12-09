use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut disk: Vec<(Option<usize>, usize)> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .flat_map(|(index, c)| {
            let size: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if index % 2 == 0 {
                vec![(Some(index / 2), size); size]
            } else {
                vec![(None, size); size]
            }
        })
        .collect();
    let (mut p1, mut p2) = (0usize, disk.len() - 1);
    let mut cur_id = disk[disk.len() - 1].0.unwrap();
    while p1 != p2 && p1 < p2 && cur_id != 0 {
        while disk[p1].0.is_some() {
            p1 += 1
        }
        while disk[p2].0.is_none() || disk[p2].0 != Some(cur_id) && p2 > 0 {
            p2 -= 1
        }
        let (lsize, rsize) = (disk[p1].1, disk[p2].1);
        if lsize >= rsize {
            for _ in 0..rsize {
                disk[p1] = disk[p2];
                disk[p2] = (None, rsize);
                p1 += 1;
                p2 -= 1;
            }

            if disk[p1].0.is_none() {
                let mut tmp = p1;
                for _ in 0..disk[p1].1 - rsize {
                    disk[tmp].1 -= rsize;
                    tmp += 1;
                }
            }
        } else {
            let mut tmp = p1;
            while tmp < p2 {
                if disk[tmp].0.is_some() || disk[tmp].1 < rsize {
                    tmp += 1;
                } else {
                    break;
                }
            }
            if tmp < p2 {
                for _ in 0..rsize {
                    disk[tmp] = disk[p2];
                    disk[p2] = (None, rsize);
                    tmp += 1;
                    p2 -= 1;
                }
                if disk[tmp].0.is_none() {
                    for _ in 0..disk[tmp].1 - rsize {
                        disk[tmp].1 -= rsize;
                        tmp += 1;
                    }
                }
            }
        }
        cur_id -= 1;
    }

    Ok(format!(
        "{}",
        disk.iter()
            .enumerate()
            .fold(0, |acc, (id, val)| acc + id * val.0.unwrap_or(0))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
