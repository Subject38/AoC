use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        let (c1, c2) = l.split('-').collect_tuple().unwrap();
        if let Some(v) = connections.get_mut(c1) {
            v.push(c2);
        } else {
            connections.insert(c1, vec![c2]);
        }
        if let Some(v) = connections.get_mut(c2) {
            v.push(c1);
        } else {
            connections.insert(c2, vec![c1]);
        }
    });
    // this is dumb and bad but it works so why not?
    // let mut res: HashSet<Vec<&str>> = HashSet::new();
    // for candidate in connections.keys().filter(|k| k.starts_with('t')) {
    //     for combo in connections.get(candidate).unwrap().iter().combinations(2) {
    //         if connections.get(combo[0]).unwrap().contains(combo[1]) {
    //             let mut thing = vec![*combo[0], *combo[1], *candidate];
    //             thing.sort();
    //             res.insert(thing);
    //         }
    //     }
    // }
    Ok(format!("0"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";
        assert_eq!("co,de,ka,ta", process(input)?);
        Ok(())
    }
}
