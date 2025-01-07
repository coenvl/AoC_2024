use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

/** Returns the location of a numerical key */
fn key_loc(c: char) -> (u8, u8) {
    // 789
    // 456
    // 123
    //  0A
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("No such button"),
    }
}

/** Get the keys needed to travel from one key to another, whilst avoiding an empty space.

    This order is always optimal for the keypad, not necessarily for the d-pad.
*/
fn travel_from_to(pos: &mut (u8, u8), dst: (u8, u8), avoid: (u8, u8)) -> String {
    let mut ret = String::new();
    if (pos.1 != avoid.1 || dst.0 != avoid.0) && pos.0 > dst.0 {
        // It is safe to go left first, then do it
        ret.extend(['<'].repeat((pos.0 - dst.0) as usize));
        pos.0 = dst.0;
    }
    if (pos.0 != avoid.0 || dst.1 != avoid.1) && pos.1 < dst.1 {
        // It is safe to go down first, then do it
        ret.extend(['v'].repeat((dst.1 - pos.1) as usize));
        pos.1 = dst.1;
    }
    ret.extend(['^'].repeat(pos.1.saturating_sub(dst.1) as usize));
    ret.extend(['>'].repeat(dst.0.saturating_sub(pos.0) as usize));
    ret.extend(['<'].repeat(pos.0.saturating_sub(dst.0) as usize));
    ret.extend(['v'].repeat(dst.1.saturating_sub(pos.1) as usize));
    dst.clone_into(pos);
    return ret + "A";
}

/** Get the buttons needed to press the keys in the code */
fn solve_keys(code: &str) -> String {
    let mut pos = key_loc('A');
    code.chars()
        .map(|c| travel_from_to(&mut pos, key_loc(c), (0, 3)))
        .collect()
}

/** Return the directional keys, to press a set of directional keys.
 
    It is not an exhaustive list, all 1 and 2 length patterns are here. Some combinations are probably
    missing for other input, but in fact not all patterns are needed.
    When extending with new patterns, they should follow same order as the 2-length patterns.
 */
fn control<'a>(pat: &'a str) -> &str {
    match pat {
        "A" => "A",
        
        "^A" => "<A>A",
        "<A" => "v<<A>>^A",
        ">A" => "vA^A",
        "vA" => "<vA^>A",

        "^^A" => "<AA>A",
        "^<A" => "<Av<A>>^A",
        "^>A" => "<Av>A^A",
        "<^A" => "v<<A>^A>A",
        "<<A" => "v<<AA>>^A",
        "<vA" => "v<<A>A^>A",
        ">^A" => "vA<^A>A",
        ">>A" => "vAA^A",
        ">vA" => "vA<A^>A",
        "v>A" => "<vA>A^A",
        "v<A" => "<vA<A>>^A",
        "vvA" => "<vAA^>A",

        "^^^A" => "<AAA>A",
        "^<<A" => "<Av<AA>>^A",
        "^>>A" => "<Av>AA^A",
        "<<^A" => "v<<AA>^A>A",
        "<^^A" => "v<<A>^AA>A",
        "<<vA" => "v<<AA>A^>A",
        "<vvA" => "v<<A>AA^>A",
        ">>^A" => "vAA<^A>A",
        ">vvA" => "vA<AA^>A",
        "v<<A" => "<vA<AA>>^A",
        "v>>A" => "<vA^AA>A",
        "vv>A" => "<vAA^A>A",
        "vvvA" => "<vAAA^>A",

        "^^<<A" => "<AAv<AA>>^A",
        "^^>>A" => "<AAv>AA^A",
        "<<^^A" => "v<<AA>^AA>A",
        "<vvvA" => "v<<A>AAA^>A",
        ">vvvA" => "vA<AAA^>A",
        ">>vvA" => "vAA<AA^>A",
        "vvv>A" => "<vAAA^A>A",

        "^^^<<A" => "<AAAv<AA>>^A",
        ">>vvvA" => "vAA<AAA^>A",
        
        _ => panic!("Pattern not covered: {}", pat),
    }
}

/** Get the number of keypresses needed for entering a code or a piece of it *N* steps away 

    Uses a hashmap for memoization to provided already known counts for a (sub)code
*/
fn get_count<'a>(pattern: &'a str, steps: u8, cache: &mut HashMap<(&'a str, u8), usize>) -> usize {
    if steps == 0 {
        // Trivial case
        pattern.len()
    } else if let Some(count) = cache.get(&(pattern, steps)) {
        // Reuse previous cached result
        *count
    } else {
        // Break it up and return sum of parts
        let count = pattern
            .split_inclusive('A')
            .map(control)
            .map(|x| get_count(x, steps - 1, cache))
            .sum();
        *cache.entry((pattern, steps)).or_insert(count)
    }
}

pub fn main() {
    let start = Instant::now();

    let direct_buttons: Vec<String> = INPUT.lines().map(solve_keys).collect();
    let numbers: Vec<usize> = INPUT
        .lines()
        .map(|line| line[..3].parse::<usize>().unwrap())
        .collect();

    let mut cache: HashMap<(&str, u8), usize> = HashMap::new();
    let buttons_1: Vec<usize> = direct_buttons
        .iter()
        .map(|code| get_count(code, 2, &mut cache))
        .collect();

    let part1: usize = buttons_1
        .iter()
        .zip(numbers.iter())
        .map(|(count, num)| num * count)
        .sum();

    println!("Part 1: {}", part1);
    assert_eq!(part1, 215374);

    let buttons_2: Vec<usize> = direct_buttons
        .iter()
        .map(|code| get_count(code, 25, &mut cache))
        .collect();

    let part2: usize = buttons_2
        .iter()
        .zip(numbers.iter())
        .map(|(count, num)| num * count)
        .sum();

    println!("Part 2: {}", part2);
    assert_eq!(part2, 260586897262600);

    println!("Time elapsed: {:?}", start.elapsed());
}
