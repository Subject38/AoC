use crate::custom_error::AocError;
use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let rows = input.lines().count() as isize;
    let cols = input.lines().next().unwrap().len() as isize;
    let nodes: Vec<(isize, isize, char)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| (i as isize, j as isize, c))
        })
        .collect();
    let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();
    for (i, j, node) in &nodes {
        for (i2, j2, node2) in &nodes {
            if (i != i2 && j != j2) && node == node2 {
                let (xdistance, ydistance) = (i - i2, j - j2);
                let (anti_i, anti_j) = (i + xdistance, j + ydistance);
                if anti_i >= 0 && anti_i < rows && anti_j >= 0 && anti_j < cols {
                    anti_nodes.insert((anti_i, anti_j));
                }
            }
        }
    }
    Ok(format!("{}", anti_nodes.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
