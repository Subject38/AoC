use crate::custom_error::AocError;
use itertools::Itertools;
use std::cmp::Ordering;

fn strength(hand: &str) -> u32 {
    let map = hand.chars().counts();
    let num_jokers = map.get(&'1').unwrap_or(&0);
    let sorted_counts: Vec<&usize> = map.values().sorted().rev().collect();
    match num_jokers {
        5 => 6,
        4 => 6,
        3 => match sorted_counts[1] {
            2 => 6,
            _ => 5,
        },
        2 => match sorted_counts[0] {
            3 => 6,
            _ => match sorted_counts[1] {
                2 => 5,
                _ => 3,
            },
        },
        1 => match sorted_counts[0] {
            4 => 6,
            3 => 5,
            2 => match sorted_counts[1] {
                2 => 4,
                _ => 3,
            },
            _ => 1,
        },
        _ => match sorted_counts[0] {
            5 => 6,
            4 => 5,
            3 => match sorted_counts[1] {
                2 => 4,
                _ => 3,
            },
            2 => match sorted_counts[1] {
                2 => 2,
                _ => 1,
            },
            _ => 0,
        },
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut hands: Vec<(String, u32, u32)> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bet)| {
            let transformed_hand = hand
                .chars()
                .map(|c| match c {
                    'T' => 'a',
                    'J' => '1',
                    'Q' => 'c',
                    'K' => 'd',
                    'A' => 'e',
                    _ => c,
                })
                .collect::<String>();
            let strength = strength(&transformed_hand);
            (transformed_hand, bet.parse().unwrap(), strength)
        })
        .collect();
    hands.sort_by(|h1, h2| {
        if h1.2 == h2.2 {
            return h1.0.cmp(&h2.0);
        }
        if h1.2 > h2.2 {
            return Ordering::Greater;
        }
        Ordering::Less
    });
    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.1 * (index + 1) as u32)
        .sum();
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
