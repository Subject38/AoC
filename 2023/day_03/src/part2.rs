use crate::custom_error::AocError;

fn parse_number(x: usize, y: usize, matrix: &[&[char]]) -> usize {
    let mut cur_x = x;
    let mut start_x = 0;
    let mut end_x = 0;
    assert!(matrix[y][x].is_ascii_digit());
    while cur_x > 0 {
        if !matrix[y][cur_x].is_ascii_digit() {
            cur_x += 1;
            start_x = cur_x;
            break;
        }
        cur_x -= 1;
    }
    cur_x = start_x;
    while cur_x < matrix[y].len() {
        if !matrix[y][cur_x].is_ascii_digit() {
            end_x = cur_x - 1;
            break;
        }
        cur_x += 1;
        end_x = matrix[y].len() - 1
    }
    matrix[y][start_x..=end_x].iter().collect::<String>().parse().unwrap()
}

fn find_number_locations(adj_list: &[Vec<char>]) -> Vec<[i32; 2]> {
    let mut sum = 0;
    let mut num_locations = vec![[-1, -1], [-1, -1]];
    if adj_list[1][0].is_ascii_digit() {
        if sum < 2 {
            num_locations[sum] = [1, 0];
        }
        sum += 1;
    }
    if adj_list[1][2].is_ascii_digit() {
        if sum < 2 {
            num_locations[sum] = [1, 2];
        }
        sum += 1;
    }
    let mut found_num = false;
    for y in [0, 2] {
        for x in 0..3 {
            if !found_num {
                if adj_list[y][x].is_ascii_digit() {
                    if sum < 2 {
                        num_locations[sum] = [y as i32, x as i32];
                    }
                    sum += 1;
                    found_num = true;
                }
            } else if !adj_list[y][x].is_ascii_digit() {
                found_num = false;
            }
        }
        found_num = false;
    }
    if sum == 2 {
        return num_locations;
    }
    vec![[-1, -1], [-1, -1]]
}

fn gear_ratio(x: usize, y: usize, matrix: &[&[char]]) -> usize {
    let mut found_digits = vec![vec!['.', '.', '.'], vec!['.', '.', '.'], vec!['.', '.', '.']];
    if y > 0 {
        if x > 0 && matrix[y-1][x-1].is_ascii_digit() {
            found_digits[0][0] = matrix[y-1][x-1];
        }
        if matrix[y-1][x].is_ascii_digit() {
            found_digits[0][1] = matrix[y-1][x];
        }
        if x < matrix[y].len() - 1 && matrix[y-1][x+1].is_ascii_digit() {
            found_digits[0][2] = matrix[y-1][x+1];
        }
    }
    if x > 0 && matrix[y][x-1].is_ascii_digit() {
        found_digits[1][0] = matrix[y][x-1];
    }
    if x < matrix[y].len() - 1 && matrix[y][x+1].is_ascii_digit() {
        found_digits[1][2] = matrix[y][x+1];
    }
    if y < matrix.len() - 1 {
        if x > 0 && matrix[y+1][x-1].is_ascii_digit() {
            found_digits[2][0] = matrix[y+1][x-1];
        }
        if matrix[y+1][x].is_ascii_digit() {
            found_digits[2][1] = matrix[y+1][x];
        }
        if x < matrix[y].len() - 1 && matrix[y+1][x+1].is_ascii_digit() {
            found_digits[2][2] = matrix[y+1][x+1];
        }
    }
    let mut gear_tuple = [0, 0];
    for (index, location) in find_number_locations(found_digits.as_slice()).iter().enumerate() {
        let [y_mod, x_mod] = location;
        if location != &[-1, -1] {
            assert!(matrix[y+*y_mod as usize - 1][x+*x_mod as usize-1].is_ascii_digit());
            gear_tuple[index] = parse_number(x+*x_mod as usize-1, y+*y_mod as usize-1, matrix)
        }
    }
    gear_tuple[0] * gear_tuple[1]
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let input_chars = input.chars().collect::<Vec<char>>();
    let char_matrix = input_chars.as_slice().split(|c| *c == '\n').collect::<Vec<_>>();
    let mut sum = 0;
    for (y, line) in char_matrix.iter().enumerate() {
        for (x, chara) in line.iter().enumerate() {
            if *chara == '*' {
                sum += gear_ratio(x, y, char_matrix.as_slice());
            }
        }
    }
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
