use crate::custom_error::AocError;
use itertools::Itertools;
use std::array::from_fn;

struct Lens {
    name: String,
    focal_len: usize,
}

fn hash(value: &str) -> u8 {
    value
        .chars()
        .fold(0_u8, |hash, c| hash.wrapping_add(c as u8).wrapping_mul(17))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut hashmap: [Vec<Lens>; 256] = from_fn(|_| Vec::new());
    input.lines().next().unwrap().split(',').for_each(|value| {
        let (val, num) = value
            .split(|c| c == '=' || c == '-')
            .collect_tuple()
            .unwrap();
        let val_hash = hash(val) as usize;
        let lens_box = &mut hashmap[val_hash];
        let lens_index = lens_box.iter().position(|lens| lens.name == val);
        if let Ok(focal_len) = num.parse::<usize>() {
            if let Some(lens_index) = lens_index {
                lens_box[lens_index].focal_len = focal_len
            } else {
                lens_box.push(Lens {
                    name: val.to_owned(),
                    focal_len,
                })
            }
        } else if let Some(lens_index) = lens_index {
            lens_box.remove(lens_index);
        }
    });
    let focus_power = hashmap
        .iter()
        .enumerate()
        .map(|(box_number, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(slot_number, lens)| lens.focal_len * (slot_number + 1) * (box_number + 1))
                .sum::<usize>()
        })
        .sum::<usize>();
    Ok(format!("{focus_power}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
