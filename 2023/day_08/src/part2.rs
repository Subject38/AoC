use crate::custom_error::AocError;
use num::integer::lcm;
use std::collections::HashMap;

struct Node {
    pub right: String,
    pub left: String,
}

enum Direction {
    Left,
    Right,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();
    use Direction::*;
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("Bruh moment"),
        })
        .collect();
    lines.next();
    let mut node_map = HashMap::new();
    lines.for_each(|line| {
        let (name, nodes) = line.split_once(" = ").unwrap();
        let (left, right) = nodes.split_once(", ").unwrap();
        let left_s = left.strip_prefix('(').unwrap();
        let right_s = right.strip_suffix(')').unwrap();
        node_map.insert(
            name.to_owned(),
            Node {
                left: left_s.to_owned(),
                right: right_s.to_owned(),
            },
        );
    });
    let count = node_map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|mut node| {
            let mut count = 0;
            while !node.ends_with('Z') {
                let direction = &directions[count % directions.len()];
                let left = &node_map.get(node).unwrap().left;
                let right = &node_map.get(node).unwrap().right;
                node = match direction {
                    Left => &left,
                    Right => &right,
                };
                count += 1;
            }
            count
        })
        .reduce(lcm)
        .unwrap();

    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
