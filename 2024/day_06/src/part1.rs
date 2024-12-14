use crate::custom_error::AocError;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Marker {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Obstacle,
    Empty,
    Visited,
    Finished,
    Invalid,
}

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn check_bounds(mat: &[Vec<Marker>], i: usize, j: usize) -> bool {
    mat.get(i).is_some_and(|row| row.get(j).is_some())
}

fn change_direction(direction: Marker) -> Marker {
    use Marker::{East, North, South, West};
    match direction {
        North => East,
        East => South,
        South => West,
        West => North,
        _ => unreachable!("not a direction"),
    }
}

fn simulate(mat: &mut [Vec<Marker>], i: usize, j: usize) -> (Marker, usize, usize) {
    use Marker::*;
    let new_loc = match mat[i][j] {
        North | East | South | West => {
            let direction = DIRECTIONS[mat[i][j] as usize];
            let (new_i, new_j) = (
                (i as isize + direction.0 as isize) as usize,
                (j as isize + direction.1 as isize) as usize,
            );
            if check_bounds(mat, new_i, new_j) {
                match mat[new_i][new_j] {
                    Obstacle => (change_direction(mat[i][j]), i, j),
                    Visited | Empty => (mat[i][j], new_i, new_j),
                    _ => {
                        unreachable!("Invalid map state")
                    }
                }
            } else {
                (Finished, mat.len(), mat[0].len()) // out of bounds
            }
        }
        _ => unreachable!("Pass in the correct guard location lol"),
    };
    mat[i][j] = Visited; // mark the previous location as now visited (this will get overwritten if we are turning)
    if new_loc != (Finished, mat.len(), mat[0].len()) {
        let (direc, i, j) = new_loc;
        mat[i][j] = direc;
    }
    new_loc
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
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
                        '^' => North,
                        '>' => East, // unused for me but shown in the spec
                        'v' => South,
                        '<' => West,
                        _ => Invalid,
                    };
                    if [North, East, South, West].contains(&marker) {
                        guard_loc = (i, j)
                    }
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
    loop {
        let new_loc = simulate(&mut thing, guard_loc.0, guard_loc.1);
        if new_loc == (Finished, thing.len(), thing[0].len()) {
            break;
        }
        guard_loc = (new_loc.1, new_loc.2)
    }
    Ok(format!(
        "{}",
        thing
            .iter()
            .flatten()
            .filter(|marker| **marker == Visited)
            .count()
    ))
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
