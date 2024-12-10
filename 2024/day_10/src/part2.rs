use crate::custom_error::AocError;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    pub r: usize,
    pub c: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            r: value.0,
            c: value.1,
        }
    }
}

impl Position {
    pub fn neighbors(self, grid: &[Vec<u32>]) -> impl Iterator<Item = Self> {
        static NEIGHBOR_POSITIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        let (r, c) = (self.r, self.c);
        let (r_len, c_len) = (grid.len(), grid[0].len());
        NEIGHBOR_POSITIONS.iter().filter_map(move |(dr, dc)| {
            let r = match dr {
                ..=-1 => r.checked_sub(1)?,
                0.. => r.checked_add(*dr as usize)?,
            };
            let c = match dc {
                ..=-1 => c.checked_sub(1)?,
                0.. => c.checked_add(*dc as usize)?,
            };
            if r < r_len && c < c_len {
                Some((r, c).into())
            } else {
                None
            }
        })
    }
}

fn find_nines(grid: &[Vec<u32>], p: Position) -> usize {
    if grid[p.r][p.c] == 9 {
        return 1;
    }
    let cur_val = grid[p.r][p.c];

    let mut res = 0;
    for n in p.neighbors(grid) {
        if grid[n.r][n.c] == cur_val + 1 {
            res += find_nines(grid, n);
        }
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut starting_points: Vec<Position> = vec![];
    let grid: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    let res = c.to_digit(10).unwrap();
                    if res == 0 {
                        starting_points.push((i, j).into());
                    }
                    res
                })
                .collect()
        })
        .collect();
    let scores = starting_points
        .iter()
        .fold(0, |acc, pos| acc + find_nines(&grid, *pos));
    Ok(format!("{}", scores))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
