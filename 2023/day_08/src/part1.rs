use crate::custom_error::AocError;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    pub right: String,
    pub left: String,
}

#[derive(Debug)]
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
        let left_s = left.strip_prefix('(').unwrap().to_owned();
        let right_s = right.strip_suffix(')').unwrap().to_owned();
        node_map.insert(name, Node { left: left_s, right: right_s });
    });
    let mut count = 0;
    let mut cur_node = "AAA";
    while cur_node != "ZZZ" {
        let direction = &directions[count % directions.len()];
        cur_node = match direction {
            Left => {
                &node_map.get(cur_node).unwrap().left
            },
            Right => {
                &node_map.get(cur_node).unwrap().right
            }
        };
        count += 1;
    }
    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
