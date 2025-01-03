const TEST: bool = true;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };
const LINE_LEN: usize = if TEST {11} else {60}; // 131 or 11
const NUM_LINES: usize = LINE_LEN;

pub fn main() {

    assert_eq!(LINE_LEN as usize, INPUT.lines().nth(0).unwrap().len() + 1);
    assert_eq!(NUM_LINES as usize, INPUT.lines().count() + 1);

    println!("Part 1: {}", total);

    println!("Part 2: {}", total2);

    // Part 1: 1223326
    // Part 2: 21070419
}