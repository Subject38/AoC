fn main() {
  let input = include_str!("../../input.txt");
  let result = part2(input);
  println!("{}", result);
}

fn read_number(line: &[char], mut index: usize) -> u32 {
  match line[index] {
    'o' => {
      if line.len() > index + 2 {
        index += 1;
        match line[index] {
          'n' => {
            index += 1;
            match line[index] {
              'e' => 1,
              _ => 0
            }
          },
          _ => 0
        }
      } else {
        0
      }
    },
    't' => {
      if line.len() > index + 2 {
        index += 1;
        match line[index] {
          'h' => {
            if line.len() > index + 3 {
              index += 1;
              match line[index] {
                'r' => {
                  index += 1;
                  match line[index] {
                    'e' => {
                      index += 1;
                      match line[index] {
                        'e' => 3,
                        _ => 0,
                      }
                    }
                    _ => 0
                  }
                }, 
                _ => 0
              }
            } else {
              0
            }
          },
          'w' => {
            index += 1;
            match line[index] {
              'o' => 2,
              _ => 0
            }
          },
          _ => 0
        }
      } else {
        0
      }
    },
    'f' => {
      if line.len() > index + 3 {
        index += 1;
        match line[index] {
          'o' => {
            index += 1;
            match line[index] {
              'u' => {
                index += 1;
                match line[index] {
                  'r' => 4,
                  _ => 0
                }
              },
              _ => 0
            }
          },
          'i' => {
            index += 1;
            match line[index] {
              'v' => {
                index += 1;
                match line[index] {
                  'e' => 5,
                  _ => 0
                }
              },
              _ => 0
            }
          }
          _ => 0
        }
      } else {
        0
      }
    },
    's' => {
      if line.len() > index + 2 {
        index += 1;
        match line[index] {
          'e' => {
            if line.len() > index + 3 {
              index += 1;
              match line[index] {
                'v' => {
                  index += 1;
                  match line[index] {
                    'e' => {
                      index += 1;
                      match line[index] {
                        'n' => 7,
                        _ => 0,
                      }
                    }
                    _ => 0
                  }
                }, 
                _ => 0
              }
            } else {
              0
            }
          },
          'i' => {
            index += 1;
            match line[index] {
              'x' => 6,
              _ => 0
            }
          },
          _ => 0
        }
      } else {
        0
      }
    },
    'e' => {
      if line.len() > index + 4 {
        index += 1;
        match line[index] {
          'i' => {
            index += 1;
            match line[index] {
              'g' => {
                index += 1;
                match line[index] {
                  'h' => {
                    index += 1;
                    match line[index] {
                      't' => 8,
                      _ => 0
                    }
                  },
                  _ => 0
                }
              },
              _ => 0
            }
          },
          _ => 0
        }
      } else {
        0
      }
    },
    'n' => {
      if line.len() > index + 3 {
        index += 1;
        match line[index] {
          'i' => {
            index += 1;
            match line[index] {
              'n' => {
                index += 1;
                match line[index] {
                  'e' => 9,
                  _ => 0,
                }
              },
              _ => 0
            }
          },
          _ => 0
        }
      } else {
        0
      }
    }
    _ => {
      0
    }
  }
}

fn part2(input: &str) -> usize {
  let mut sum = 0;
  for line in input.split("\n") {
    let characters: Vec<char> = line.chars().collect();
    let mut found_first: u8 = 0;
    let mut cur_num = 0;
    let mut first_digit = 0;
    for (index, character) in characters.iter().enumerate() {
      if character.is_ascii_digit() {
        cur_num = character.to_digit(10).unwrap();
        if found_first == 0 {
          found_first = 1;
          first_digit = cur_num;
        }
      }
      let tmp_num = read_number(&characters, index);
      if tmp_num != 0 {
        cur_num = tmp_num;
        if found_first == 0 {
          found_first = 1;
          first_digit = cur_num;
        }
      }
    }
    let last_digit = cur_num;
    sum += first_digit * 10 + last_digit;
  }
  sum as usize
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_2() {
    let result = part2(
      "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"
    );
    assert_eq!(result, 281)
  }
}