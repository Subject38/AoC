use crate::custom_error::AocError;

fn get_points(line: &str) -> usize {
    let mut found_nums = 0;
    let (_, line_contents) = line.split_once(':').unwrap();
    let (winning_nums_str, my_nums_str) = line_contents.split_once('|').unwrap();
    let winning_nums: Vec<usize> = winning_nums_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let my_nums: Vec<usize> = my_nums_str
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for num in my_nums {
        if winning_nums.contains(&num) {
            found_nums += 1;
        }
    }
    found_nums
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut card_totals = vec![1; input.lines().collect::<Vec<&str>>().len()];
    for (index, line) in input.lines().enumerate() {
        let found_nums = get_points(line);
        let num_to_add = card_totals[index];
        for item in card_totals
            .iter_mut()
            .take(index + found_nums + 1)
            .skip(index + 1)
        {
            *item += num_to_add;
        }
    }
    Ok(format!("{}", card_totals.iter().sum::<usize>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
