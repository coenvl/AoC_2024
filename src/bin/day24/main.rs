use std::collections::{HashMap, HashSet};

const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};

const AND: &str = " AND ";
const OR: &str = " OR ";
const XOR: &str = " XOR ";

type Ops = HashMap<&'static str, (&'static str, &'static str, &'static str)>;

fn init_state(input: &'static str) -> HashMap<&'static str, bool> {
    input
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").unwrap();
            (key, value == "1")
        })
        .collect()
}

fn parse_ops(input: &'static str) -> Ops {
    input
        .lines()
        .map(|line| {
            let (expr, target) = line.split_once(" -> ").unwrap();
            for op in [AND, OR, XOR] {
                if expr.contains(op) {
                    let args = expr.split_once(op).unwrap();
                    return (target, (args.0, op, args.1));
                }
            }
            panic!("Unknown operator");
        })
        .collect()
}

fn get_value(wire: &'static str, state: &mut HashMap<&str, bool>, ops: &Ops) -> bool {
    if let Some(&value) = state.get(wire) {
        value
    } else if let Some(op) = ops.get(wire) {
        let value = match op.1 {
            AND => get_value(op.0, state, ops) && get_value(op.2, state, ops),
            OR => get_value(op.0, state, ops) || get_value(op.2, state, ops),
            XOR => get_value(op.0, state, ops) ^ get_value(op.2, state, ops),
            _ => panic!("Unknown operator in compute"),
        };
        state.insert(wire, value);
        value
    } else {
        panic!("Wire not found!")
    }
}

fn get_bitnum(state: &HashMap<&str, bool>, prefix: char) -> u128 {
    let mut keys: Vec<&&str> = state.keys().filter(|&x| x.starts_with(prefix)).collect();
    keys.sort();
    keys.iter().map(|&z| *state.get(z).unwrap())
        .enumerate()
        .map(|(i, b)| if b { 2_u128.pow(i as u32) } else { 0 })
        .sum()
}

fn is_half_adder(wire: &str, ops: &Ops) -> bool {
    let op = ops.get(wire).unwrap();
    op.1 == XOR
}

fn is_primitive(op: &(&str, &str, &str)) -> bool {
    (op.0.starts_with('x') || op.0.starts_with('y'))
        && (op.2.starts_with('x') || op.2.starts_with('y'))
}

fn as_carry(wire: &'static str, ops: &Ops, len: u8) -> Option<&'static str> {
    let op = ops.get(wire).unwrap();
    if is_primitive(op) {
        if len != 0 {
            Some(wire)
        } else if op.0 == "x00" || op.0 == "y00" {
            if op.1 != AND { Some(wire) } else { None }
        } else {
            if op.1 != XOR { Some(wire) } else { None }
        }
    } else if op.1 != OR {
        Some(wire)
    } else {
        as_carry_part(op.2, ops, len).or_else(|| as_carry_part(op.0, ops, len))
    }
}

fn as_carry_part(wire: &'static str, ops: &Ops, len: u8) -> Option<&'static str> {
    // Part of the carry side of OR
    let op = ops.get(wire).unwrap();
    if op.1 != AND {
        Some(wire)
    } else if is_primitive(op) {
        let n = op.0[1..].parse::<u8>().unwrap();
        if len != n { Some(wire) } else { None }
    } else if is_primitive(ops.get(op.0).unwrap()) {
        as_half_adder(op.0, ops, len).or_else(|| as_carry(op.2, ops, len - 1))
    } else {
        as_half_adder(op.2, ops, len).or_else(|| as_carry(op.0, ops, len - 1))
    }
}

fn as_half_adder(wire: &'static str, ops: &Ops, len: u8) -> Option<&'static str> {
    let op = ops.get(wire).unwrap();
    if is_primitive(op) {
        let n = op.0[1..].parse::<u8>().unwrap();
        if (n == 0 && len != 1) || (n > 0 && len != n) { Some(wire) } else { None }
    } else if op.1 != XOR {
        Some(wire)
    } else if is_half_adder(op.0, ops) {
        as_half_adder(op.0, ops, len).or_else(|| as_carry_part(op.2, ops, len - 1))
    } else {
        as_half_adder(op.2, ops, len).or_else(|| as_carry_part(op.0, ops, len - 1))
    }
}

fn as_full_adder(wire: &'static str, ops: &Ops, len: u8) -> Option<&'static str> {
    let op = ops.get(wire).unwrap();
    if is_primitive(op) {
        if len != 0 { Some(wire) } else { None }
    } else if op.1 != XOR {
        Some(wire)
    } else if is_half_adder(op.0, ops) {
        as_half_adder(op.0, ops, len).or_else(|| as_carry(op.2, ops, len - 1))
    } else {
        as_half_adder(op.2, ops, len).or_else(|| as_carry(op.0, ops, len - 1))
    }
}

fn search_wires(wire: &'static str, ops: &Ops) -> HashSet<&'static str> {
    if wire.starts_with('z') {
        HashSet::new()
    } else {
        let mut ret = HashSet::from([wire]);
        ops.iter()
            .filter(|(&_k, &v)| v.0 == wire || v.2 == wire)
            .flat_map(|(&k, &_v)| search_wires(k, ops))
            .for_each(|k| {
                ret.insert(k);
            });
        ret
    }
}

fn inv_search_wires(wire: &'static str, ops: &Ops) -> HashSet<&'static str> {
    if wire.starts_with('x') || wire.starts_with('y') {
        HashSet::new()
    } else {
        let mut ret = HashSet::from([wire]);
        let op = ops.get(&wire).unwrap();
        ret.extend(inv_search_wires(op.0, ops));
        ret.extend(inv_search_wires(op.2, ops));
        ret
    }
}

fn find_swap(
    zwire: &'static str,
    len: u8,
    bug: &'static str,
    candidates: HashSet<&'static str>,
    ops: &Ops,
) -> &'static str {
    let mut op2 = ops.clone();
    for s in candidates
        .iter()
        .filter(|&&x| x != bug && !x.starts_with('x') && !x.starts_with('y'))
    {
        key_swap(bug, s, &mut op2);

        let another_bug = as_full_adder(zwire, &op2, len);
        if another_bug.is_none() {
            // We have found the correct swap
            return s;
        }
        key_swap(bug, s, &mut op2); // swap back
    }
    panic!("No swappable element found!");
}

fn key_swap(a: &'static str, b: &'static str, map: &mut Ops) {
    let temp0 = map
        .remove(a)
        .unwrap_or_else(|| panic!("Unable to swap items"));
    let temp1 = map
        .remove(b)
        .unwrap_or_else(|| panic!("Unable to swap items"));
    map.insert(b, temp0);
    map.insert(a, temp1);
}

pub fn main() {
    let inputs: Vec<&str> = INPUT.split("\n\n").collect();
    let mut state = init_state(inputs[0]);
    let ops: Ops = parse_ops(inputs[1]);

    let mut zwires: Vec<&&str> = ops.keys().filter(|x| x.starts_with('z')).collect();
    zwires.sort();
    zwires.iter().for_each(|&z| {
        get_value(z, &mut state, &ops);
    });
    println!("Part 1: {}", get_bitnum(&state, 'z'));

    let mut swaps: Vec<&str> = Vec::new(); //vec![("z07", "swt"), ("z13", "pqc"), ("rjm", "wsv"), ("z31", "bgs")];
    let mut op2: Ops = ops.clone();

    for (i, &&zwire) in zwires.iter().enumerate() {
        if i > 44 {
            continue;
        }
        let result: Option<&str> = as_full_adder(zwire, &op2, i as u8);
        if result.is_some() {
            let mut wires = search_wires(Box::leak(format!("x{:02}", i).into_boxed_str()), &op2);
            wires.extend(inv_search_wires(
                Box::leak(format!("z{:02}", i).into_boxed_str()),
                &op2,
            ));

            let bug2 = find_swap(zwire, i as u8, result.unwrap(), wires, &op2);
            key_swap(result.unwrap(), bug2, &mut op2);
            swaps.push(result.unwrap());
            swaps.push(bug2);
        }
    }

    swaps.sort();
    println!("Part 2: {}", swaps.join(","));
    assert_eq!(
        swaps,
        ["bgs", "pqc", "rjm", "swt", "wsv", "z07", "z13", "z31"]
    )
}
