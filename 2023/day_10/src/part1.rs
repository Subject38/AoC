use crate::custom_error::AocError;

fn get_possible_directions(adjacent_chars: &[char], cur_char: char) -> Vec<usize> {
    let (north, east, south, west) = (0, 1, 2, 3);
    let mut possible_directions = Vec::new();
    for (direction, c) in adjacent_chars.iter().enumerate() {
        if direction == north {
            match cur_char {
                '-' | 'F' | '7' => {}
                _ => match c {
                    '|' => {
                        possible_directions.push(direction);
                    }
                    '7' => {
                        possible_directions.push(direction);
                    }
                    'F' => {
                        possible_directions.push(direction);
                    }
                    _ => {}
                },
            }
        } else if direction == east {
            match cur_char {
                '|' | 'J' | '7' => {}
                _ => match c {
                    '7' => {
                        possible_directions.push(direction);
                    }
                    '-' => {
                        possible_directions.push(direction);
                    }
                    'J' => {
                        possible_directions.push(direction);
                    }
                    _ => {}
                },
            }
        } else if direction == south {
            match cur_char {
                '-' | 'J' | 'L' => {}
                _ => match c {
                    '|' => {
                        possible_directions.push(direction);
                    }
                    'L' => {
                        possible_directions.push(direction);
                    }
                    'J' => {
                        possible_directions.push(direction);
                    }
                    _ => {}
                },
            }
        } else if direction == west {
            match cur_char {
                '|' | 'L' | 'F' => {}
                _ => match c {
                    '-' => {
                        possible_directions.push(direction);
                    }
                    'L' => {
                        possible_directions.push(direction);
                    }
                    'F' => {
                        possible_directions.push(direction);
                    }
                    _ => {}
                },
            }
        }
        if *c == 'S' {
            possible_directions.push(direction)
        }
    }
    possible_directions
}

fn get_adjacent_chars(y: usize, x: usize, grid: &[&[char]]) -> [char; 4] {
    let north = if y == 0 { '.' } else { grid[y - 1][x] };
    let east = if x == grid[0].len() - 1 {
        '.'
    } else {
        grid[y][x + 1]
    };
    let south = if y == grid.len() - 1 {
        '.'
    } else {
        grid[y + 1][x]
    };
    let west = if x == 0 { '.' } else { grid[y][x - 1] };

    [north, east, south, west]
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input_chars = input.chars().collect::<Vec<char>>();
    let grid = input_chars.split(|c| *c == '\n').collect::<Vec<_>>();
    let (start_y, start_x) = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            if let Some((x, _)) = line.iter().enumerate().find(|(_, c)| **c == 'S') {
                return Some((y, x));
            }
            None
        })
        .expect("Start position not found. There is no S character in the grid");
    let mut adjacent_chars = get_adjacent_chars(start_y, start_x, &grid);
    let direction_offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut possible_directions = get_possible_directions(&adjacent_chars, 'S');
    // arbitrarily pick the first possible direction for this iteration
    let mut direction = possible_directions[0];
    let mut cur_char = adjacent_chars[direction];
    let (mut cur_y, mut cur_x) = (
        start_y as i32 + direction_offsets[direction].0,
        start_x as i32 + direction_offsets[direction].1,
    );
    let mut count = 1;
    while cur_char != 'S' {
        let from_direction = match direction {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => unreachable!(),
        };
        adjacent_chars = get_adjacent_chars(cur_y as usize, cur_x as usize, &grid);
        possible_directions = get_possible_directions(&adjacent_chars, cur_char);
        direction = *possible_directions
            .iter()
            .find(|dir| **dir != from_direction)
            .expect("At least one direction should be valid");
        cur_char = adjacent_chars[direction];
        (cur_y, cur_x) = (
            cur_y + direction_offsets[direction].0,
            cur_x + direction_offsets[direction].1,
        );
        count += 1;
    }
    println!("{count}");
    Ok(format!("{}", (count) / 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() -> miette::Result<()> {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_2() -> miette::Result<()> {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
