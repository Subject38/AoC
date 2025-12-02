use crate::custom_error::AocError;

fn check_lock_key(lock: &[i32; 5], key: &[i32; 5]) -> bool {
    for (i, j) in lock.iter().zip(key.iter()) {
        if i + j > 5 {
            return false;
        }
    }
    true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut locks = vec![];
    let mut keys = vec![];
    input.split("\n\n").for_each(|item| {
        let mut counts = [0; 5];
        for line in item.lines().skip(1).take(5) {
            for (index, c) in line.chars().take(5).enumerate() {
                // guaranteed to be 5 anyways but to be safe...
                counts[index] += if c == '#' { 1 } else { 0 }
            }
        }
        // determine if lock or key
        match item.chars().next().unwrap() {
            '#' => locks.push(counts),
            '.' => keys.push(counts),
            _ => unreachable!("lol"),
        }
    });
    let mut result = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if check_lock_key(lock, key) {
                result += 1
            }
        }
    }
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
