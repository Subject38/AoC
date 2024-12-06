use crate::custom_error::AocError;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Marker {
    Guard(Direction),
    Obstacle,
    Empty,
    Visited(Direction),
    Finished,
    Invalid,
}

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn check_bounds(mat: &[Vec<Marker>], i: usize, j: usize) -> bool {
    match mat.get(i) {
        Some(row) => match row.get(j) {
            Some(_) => true,
            None => false,
        },
        None => false,
    }
}

fn change_direction(direction: Direction) -> Direction {
    use Direction::{East, North, South, West};
    match direction {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

fn simulate(mat: &mut Vec<Vec<Marker>>, i: usize, j: usize) -> (Marker, usize, usize, bool) {
    // this code is a failed attempt to avoid having to simulate the loops for every location.
    // as far as i can tell this approach is unfixably broken due to several edge cases
    // i didn't consider until it was too late... as such, i am declaring defeat on this problem
    // i don't want to do cycle detection today...
    use Marker::*;
    match mat[i][j] {
        Guard(dir) => {
            let direction = DIRECTIONS[dir as usize];
            let (new_i, new_j) = (
                (i as isize + direction.0 as isize) as usize,
                (j as isize + direction.1 as isize) as usize,
            );
            if check_bounds(&mat, new_i, new_j) {
                match mat[new_i][new_j] {
                    Obstacle => {
                        let res = (Guard(change_direction(dir)), i, j, false);
                        mat[i][j] = res.0;
                        res
                    }
                    Empty => {
                        let mut res = (mat[i][j], new_i, new_j, false);
                        mat[i][j] = Visited(dir);
                        mat[new_i][new_j] = res.0;
                        let possible = change_direction(dir);
                        let offsets = DIRECTIONS[possible as usize];
                        let (possible_i, possible_j) = (
                            (new_i as isize + offsets.0 as isize) as usize,
                            (new_j as isize + offsets.1 as isize) as usize,
                        );
                        if check_bounds(&mat, possible_i, possible_j)
                            && Visited(possible) == mat[possible_i][possible_j]
                        {
                            res.3 = true
                        }
                        res
                    }
                    Visited(other_dir) => {
                        let res = if change_direction(dir) == other_dir {
                            (mat[i][j], new_i, new_j, true)
                        } else {
                            (mat[i][j], new_i, new_j, false)
                        };
                        mat[i][j] = Visited(dir);
                        mat[new_i][new_j] = res.0;
                        res
                    }
                    _ => {
                        unreachable!("Invalid map state")
                    }
                }
            } else {
                (Finished, mat.len(), mat[0].len(), false) // out of bounds
            }
        }
        _ => unreachable!("Pass in the correct guard location lol"),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    use Direction::*;
    use Marker::*;
    let mut guard_loc = (input.len(), input.len()); // super invalid value.
    let mut thing: Vec<Vec<Marker>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    let marker = match c {
                        '.' => Empty,
                        '#' => Obstacle,
                        '^' => Guard(North),
                        '>' => Guard(East), // unused for me but shown in the spec
                        'v' => Guard(South),
                        '<' => Guard(West),
                        _ => Invalid,
                    };
                    match marker {
                        Guard(_) => guard_loc = (i, j),
                        _ => {}
                    };
                    marker
                })
                .collect()
        })
        .collect();
    assert_ne!(
        guard_loc,
        (input.len(), input.len()),
        "This should be initialized to something valid..."
    );
    let mut res = 0;
    loop {
        let new_loc = simulate(&mut thing, guard_loc.0, guard_loc.1);
        if new_loc == (Finished, thing.len(), thing[0].len(), false) {
            break;
        }
        if new_loc.3 {
            res += 1;
        }
        guard_loc = (new_loc.1, new_loc.2)
    }
    Ok(format!("{res}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
