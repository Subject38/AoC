fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split("\n") {
        let characters: Vec<char> = line.chars().collect();
        let mut first_digit = 0;
        let mut last_digit = 0;
        for character in characters.iter() {
            if character.is_ascii_digit() {
                first_digit = character.to_digit(10).unwrap();
                break
            }
        }
        for character in characters.iter().rev() {
            if character.is_ascii_digit() {
                last_digit = character.to_digit(10).unwrap();
                break
            }
        }
        sum += first_digit * 10 + last_digit;
    }
    sum as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"
        );
        assert_eq!(result, 142)
    }
}