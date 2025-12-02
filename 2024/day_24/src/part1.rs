use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unreachable!("lol"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Expression<'a> {
    opl: &'a str,
    opr: &'a str,
    op: Operation,
}

fn resolve_operand<'a>(
    operand: &str,
    expressions: &HashMap<&str, Expression<'a>>,
    state: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(val) = state.get(operand) {
        *val
    } else {
        let expr = expressions[operand];
        if state.get(expr.opl).is_none() {
            let opl = resolve_operand(expr.opl, expressions, state);
            state.insert(expr.opl, opl);
        }
        if state.get(expr.opr).is_none() {
            let opr = resolve_operand(expr.opr, expressions, state);
            state.insert(expr.opr, opr);
        }
        match expr.op {
            Operation::AND => state[expr.opl] && state[expr.opr],
            Operation::OR => state[expr.opl] || state[expr.opr],
            Operation::XOR => state[expr.opl] ^ state[expr.opr],
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (initial_state_str, actions_str) = input.split("\n\n").collect_tuple().unwrap();
    let mut state: HashMap<&str, bool> = initial_state_str
        .lines()
        .map(|l| {
            let (reg, val) = l.split(": ").collect_tuple().unwrap();
            (reg, val.parse::<usize>().unwrap() != 0)
        })
        .collect();
    let actions: HashMap<&str, Expression> = actions_str
        .lines()
        .map(|l| {
            let (expr, wire) = l.split(" -> ").collect_tuple().unwrap();
            let (operand_one, action, operand_two) = expr.split(' ').collect_tuple().unwrap();
            (
                wire,
                Expression {
                    opl: operand_one,
                    opr: operand_two,
                    op: action.into(),
                },
            )
        })
        .collect();
    for z in actions.keys().filter(|k| k.starts_with('z')) {
        let res = resolve_operand(z, &actions, &mut state);
        state.insert(z, res);
    }
    let mut res = 0;
    for (index, z) in state
        .keys()
        .filter(|k| k.starts_with('z'))
        .sorted_unstable()
        .enumerate()
    {
        res += (state[z] as usize) << index
    }
    Ok(format!("{res}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_trivial() -> miette::Result<()> {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!("4", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_bigger() -> miette::Result<()> {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!("2024", process(input)?);
        Ok(())
    }
}
