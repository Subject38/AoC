use crate::custom_error::AocError;

#[tracing::instrument]
fn parse_line(
    line: &str
) -> usize {
    let num_red: usize = 12;
    let num_green: usize = 13;
    let num_blue: usize = 14;
    let mut split_text = line.split(": ");
    let game_id_str = split_text.next().unwrap();
    let id: usize = game_id_str.split(' ').next_back().unwrap().parse().unwrap();
    let game_contents = split_text.next().unwrap();
    for round in game_contents.split("; ") {
        let elements: Vec<&str> = round.split(", ").collect();
        for element in elements {
            let mut color_pair = element.split(' ');
            let num_of_color = color_pair.next().unwrap().parse::<usize>().unwrap();
            let color = color_pair.next().unwrap();
            match color {
                "red" => {
                    if num_of_color > num_red {
                        return 0;
                    }
                },
                "green" => {
                    if num_of_color > num_green {
                        return 0;
                    }
                },
                "blue" => {
                    if num_of_color > num_blue {
                        return 0;
                    }
                },
                _ => unreachable!()
            }
        }
    }
    id
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    Ok(format!("{}", input.lines().map(parse_line).sum::<usize>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
