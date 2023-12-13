use crate::custom_error::AocError;
use itertools::Itertools;

fn find_empty_rows(grid: &[&[char]]) -> Vec<usize> {
    let mut res = Vec::new();
    grid.iter().enumerate().for_each(|(index, row)| {
        if !row.contains(&'#') {
            res.push(index);
        }
    });
    res
}

fn find_empty_cols(grid: &[&[char]]) -> Vec<usize> {
    let mut res = Vec::new();
    let num_cols = grid[0].len();
    for col in 0..num_cols {
        let mut found_galaxy = false;
        for row in grid {
            if !row.is_empty() && row[col] == '#' {
                found_galaxy = true;
            }
        }
        if !found_galaxy {
            res.push(col)
        }
    }
    res
}

fn get_real_coord(empty: &[usize], coord: usize) -> usize {
    empty
        .iter()
        .map(|empty_row_or_col| {
            if coord > *empty_row_or_col {
                return 1;
            }
            0
        })
        .sum::<usize>()
        + coord
}

fn find_galaxy_locs(
    grid: &[&[char]],
    empty_cols: &[usize],
    empty_rows: &[usize],
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == '#' {
                let real_x = get_real_coord(empty_cols, x);
                let real_y = get_real_coord(empty_rows, y);
                res.push((real_y, real_x));
            }
        })
    });
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input_chars = input.chars().collect::<Vec<char>>();
    let grid = input_chars.split(|c| *c == '\n').collect::<Vec<_>>();
    let empty_rows = find_empty_rows(&grid);
    let empty_cols = find_empty_cols(&grid);
    let galaxy_locs = find_galaxy_locs(&grid, &empty_cols, &empty_rows);
    let sum = galaxy_locs
        .iter()
        .combinations(2)
        .map(|combination| {
            let (y1, x1) = combination[0];
            let (y2, x2) = combination[1];
            let dist_x = if x2 > x1 { x2 - x1 } else { x1 - x2 };
            let dist_y = if y2 > y1 { y2 - y1 } else { y1 - y2 };
            dist_x + dist_y
        })
        .sum::<usize>();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
