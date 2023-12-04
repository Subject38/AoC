use crate::custom_error::AocError;

fn check_for_symbols(start_x: usize, end_x: usize, y: usize, matrix: &[&[char]]) -> usize {
    let parsed_num = matrix[y][start_x..=end_x]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();
    for x in start_x..=end_x {
        if x == start_x {
            if y > 0 {
                if matrix[y - 1][x] != '.' {
                    return parsed_num;
                }
                if x > 0 && matrix[y - 1][x - 1] != '.' {
                    return parsed_num;
                }
            }
            if x > 0 && matrix[y][x - 1] != '.' {
                return parsed_num;
            }
            if y < matrix.len() - 1 {
                if matrix[y + 1][x] != '.' {
                    return parsed_num;
                }
                if x > 0 && matrix[y + 1][x - 1] != '.' {
                    return parsed_num;
                }
            }
        }
        if x == end_x {
            if y > 0 {
                if matrix[y - 1][x] != '.' {
                    return parsed_num;
                }
                if x < matrix[y].len() - 1 && matrix[y - 1][x + 1] != '.' {
                    return parsed_num;
                }
            }
            if x < matrix[y].len() - 1 && matrix[y][x + 1] != '.' {
                return parsed_num;
            }
            if y < matrix.len() - 1 {
                if matrix[y + 1][x] != '.' {
                    return parsed_num;
                }
                if x < matrix[y].len() - 1 && matrix[y + 1][x + 1] != '.' {
                    return parsed_num;
                }
            }
        }
        if !(x == start_x || x == end_x) {
            if y > 0 && matrix[y - 1][x] != '.' {
                return parsed_num;
            }
            if y < matrix.len() - 1 && matrix[y + 1][x] != '.' {
                return parsed_num;
            }
        }
    }
    0
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let input_chars = input.chars().collect::<Vec<char>>();
    let char_matrix = input_chars
        .as_slice()
        .split(|c| *c == '\n')
        .collect::<Vec<_>>();
    let mut num_start_x = 0;
    let mut sum = 0;
    let mut found_num = false;
    for (y, line) in char_matrix.iter().enumerate() {
        for (x, chara) in line.iter().enumerate() {
            match found_num {
                false => {
                    if chara.is_ascii_digit() {
                        found_num = true;
                        num_start_x = x;
                        if x == line.len() - 1 {
                            let to_add = check_for_symbols(x, x, y, char_matrix.as_slice());
                            sum += to_add;
                            found_num = false;
                        }
                    }
                }
                true => {
                    if !chara.is_ascii_digit() {
                        let to_add =
                            check_for_symbols(num_start_x, x - 1, y, char_matrix.as_slice());
                        sum += to_add;
                        found_num = false;
                    } else if x == line.len() - 1 {
                        let to_add = check_for_symbols(num_start_x, x, y, char_matrix.as_slice());
                        sum += to_add;
                        found_num = false;
                    }
                }
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
        let input = "467....114
...*......
1.35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
