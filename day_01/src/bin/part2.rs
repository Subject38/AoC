fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{}", result);
}

macro_rules! parse_num {
    ($chars:expr, {$($body:tt)*}) => {
        {
            parse_num! { @parse $chars, {}, $($body)* }
        }
    };
    (@parse $chars:expr, {$($arms:tt)*}, $(,)*) => {
        match $chars.peek() {
            $($arms)*
            _ => 0
        }
    };
    (@parse $chars:expr, {$($arms:tt)*}, _ => $e:expr $(,)*) => {
        match $chars.peek() {
            $($arms)*
            _ => ($e)
        }
    };
    (@parse $chars:expr, {$($arms:tt)*}, $p:pat => { $($block:tt)* }, $($tail:tt)*) => {
        parse_num! {
            @parse
            $chars,
            {
                $($arms)*
                Some(&$p) => {
                    $chars.next().unwrap();
                    parse_num!(@parse $chars, {}, $($block)*)
                },
            },
            $($tail)*
        }
    };
    (@parse $chars:expr, {$($arms:tt)*}, $p:pat => $e:expr, $($tail:tt)*) => {
        parse_num! {
            @parse
            $chars,
            {
                $($arms)*
                Some(&$p) => ($e),
            },
            $($tail)*
        }
    };
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split("\n") {
        let characters: Vec<char> = line.chars().collect();
        let mut found_first: u8 = 0;
        let mut cur_num = 0;
        let mut first_digit = 0;
        for index in 0..characters.len() {
            let mut chars_iter = characters[index..].iter().peekable();
            let tmp_num = parse_num!(chars_iter, {
                'o' => {
                    'n' => {
                        'e' => 1,
                    },
                },
                't' => {
                    'w' => {
                        'o' => 2,
                    },
                    'h' => {
                        'r' => {
                            'e' => {
                                'e' => 3,
                            },
                        },
                    },
                },
                'f' => {
                    'o' => {
                        'u' => {
                            'r' => 4,
                        },
                    },
                    'i' => {
                        'v' => {
                            'e' => 5,
                        },
                    },
                },
                's' => {
                    'i' => {
                        'x' => 6,
                    },
                    'e' => {
                        'v' => {
                            'e' => {
                                'n' => 7,
                            },
                        },
                    },
                },
                'e' => {
                    'i' => {
                        'g' => {
                            'h' => {
                                't' => 8,
                            },
                        },
                    },
                },
                'n' => {
                    'i' => {
                        'n' => {
                            'e' => 9,
                        },
                    },
                },
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
            });
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
