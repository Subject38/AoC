use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct InstructionIter<'a> {
    inner: Chars<'a>,
}

impl<'a> InstructionIter<'a> {
    pub fn new(instruction_str: &'a str) -> InstructionIter {
        InstructionIter {
            inner: instruction_str.chars(),
        }
    }
}

impl<'a> Iterator for InstructionIter<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(c) => match c {
                '>' => Some(Direction::East),
                '^' => Some(Direction::North),
                '<' => Some(Direction::West),
                'v' => Some(Direction::South),
                '\n' => self.next(),
                _ => unreachable!("invalid instruction"),
            },
            None => None,
        }
    }
}
