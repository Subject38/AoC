use crate::custom_error::AocError;
use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};

fn check_rows(matrix: &[Vec<&char>], pattern: &[char]) -> usize {
    let mut res = 0;
    for row in matrix {
        let mut cur_index = 0;
        for c in row {
            if **c == pattern[cur_index] {
                cur_index += 1
            } else {
                cur_index = if **c == pattern[0] { 1 } else { 0 }
            }
            if cur_index == pattern.len() {
                cur_index = 0;
                res += 1
            }
        }
    }
    res
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // input is always a square matrix
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    // look for XMAS
    let xmas = ['X', 'M', 'A', 'S'];
    let samx = ['S', 'A', 'M', 'X'];
    let matrices = [
        straight_x(&matrix),
        straight_y(&matrix),
        diagonal_pos_pos(&matrix),
        diagonal_pos_neg(&matrix),
    ];
    let count = matrices.iter().fold(0, |acc, mat| {
        acc + check_rows(mat, &xmas) + check_rows(mat, &samx)
    });
    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        let new = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!("4", process(new)?);
        Ok(())
    }
}
