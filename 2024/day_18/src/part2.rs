use itertools::Itertools;

use crate::custom_error::AocError;
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbors(&self, grid: &[Vec<bool>]) -> Vec<Self> {
        static NEIGHBOR_POSITIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        let (r, c) = (self.0, self.1);
        let (r_len, c_len) = (grid.len(), grid[0].len());
        NEIGHBOR_POSITIONS
            .iter()
            .filter_map(|(dr, dc)| {
                let r = match dr {
                    ..=-1 => r.checked_sub(1)?,
                    0.. => r.checked_add(*dr as usize)?,
                };
                let c = match dc {
                    ..=-1 => c.checked_sub(1)?,
                    0.. => c.checked_add(*dc as usize)?,
                };
                if r < r_len && c < c_len && !grid[r][c] {
                    Some(Self(r, c))
                } else {
                    None
                }
            })
            .collect()
    }
}

static mut GRID_SIZE: usize = 71;
static mut BYTES_FALLEN: usize = 1024;
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // safety: GRID_SIZE is only ever read once here. this will not be called multiple times.
    let grid_size = unsafe { GRID_SIZE };
    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_size]; grid_size];
    let fallen = unsafe { BYTES_FALLEN };
    input.lines().take(fallen).for_each(|l| {
        let (c, r) = l
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        grid[r][c] = true;
    });
    let goal = Pos(grid_size - 1, grid_size - 1);
    let mut lines = input.lines().skip(fallen);
    let mut res = (0, 0);
    while bfs(&Pos(0, 0), |p| p.neighbors(&grid), |p| *p == goal).is_some() {
        let (c, r) = lines
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        res = (c, r);
        grid[r][c] = true;
    }

    Ok(format!("{},{}", res.0, res.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // safety, only write GRID_SIZE and BYTES_FALLEN here...
        unsafe { GRID_SIZE = 7 };
        unsafe { BYTES_FALLEN = 12 }
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(input)?);
        Ok(())
    }
}
