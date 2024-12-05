use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    rules_str.lines().for_each(|rule_str| {
        let (val1, val2) = rule_str
            .split('|')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .expect("malformed input");
        if let Some(rule) = rules.get_mut(&val1) {
            rule.insert(val2);
        } else {
            rules.insert(val1, HashSet::from([val2]));
        }
    });
    let mut updates: Vec<Vec<usize>> = updates_str
        .lines()
        .map(|update| {
            update
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let mut invalid_indices = vec![];
    for (index, update) in updates.iter().enumerate() {
        let mut seen: Vec<&usize> = vec![];
        'outer: for val in update {
            let rule = match &rules.get(val) {
                Some(rule) => rule,
                None => &HashSet::<usize>::new(),
            };
            for i in &seen {
                if rule.contains(i) {
                    invalid_indices.push(index);
                    break 'outer;
                }
            }
            seen.push(val)
        }
    }
    let mut res = 0;
    for i in invalid_indices {
        let update = updates.get_mut(i).unwrap();
        update.sort_unstable_by(|a, b| {
            let rule = match &rules.get(a) {
                Some(rule) => rule,
                None => &HashSet::<usize>::new(),
            };
            if rule.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        res += update[update.len() / 2]
    }
    Ok(format!("{res}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
