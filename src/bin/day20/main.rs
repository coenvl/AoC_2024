const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};
const SIZE: usize = if TEST { 16 } else { 142 }; // 131 or 11
const MIN_REDUCTION: usize = if TEST { 50 } else { 100 };

type Loc = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (0, -1);
const RIGHT: Dir = (1, 0);
const DOWN: Dir = (0, 1);
const LEFT: Dir = (-1, 0);

fn step(pos: Loc, &dir: &Dir) -> Option<Loc> {
    match dir {
        UP if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        DOWN if pos.1 < SIZE => Some((pos.0, pos.1 + 1)),
        LEFT if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        RIGHT if pos.0 < SIZE => Some((pos.0 + 1, pos.1)),
        _ => None,
    }
}

// 46 = '.'
// 35 = '#'

fn walk(start: Loc) -> Vec<Loc> {
    let bytes = INPUT.as_bytes();
    assert_eq!(bytes[start.0 + start.1*SIZE], b'S');

    let mut pos = start.clone();
    let mut prev_pos = start.clone();
    let mut route = vec![start];

    for direction in [UP, RIGHT, DOWN, LEFT].iter().cycle() {
        if let Some(location) = step(pos, direction) {
            let tile = bytes[location.0 + location.1 * SIZE];
            if location == prev_pos || tile == b'#' {
                continue;
            }
            route.push(location);
            prev_pos = pos;
            pos = location;
            if tile == b'E' {
                return route;
            }
        }
    }
    return vec![]; // Here we failed to find a route
}

fn find_reductions(route: &[Loc], max_length: usize) -> usize {
    let mut count = 0;
    // let mut ret: Vec<usize> = vec![];
    for i in 0..route.len()-MIN_REDUCTION {
        for j in MIN_REDUCTION+i..route.len() {
            let cheat_length = route[i].0.abs_diff(route[j].0) + route[i].1.abs_diff(route[j].1);
            let reduction = (j-i) - cheat_length;
            if reduction >= MIN_REDUCTION && cheat_length <= max_length {
                // ret.push(reduction);
                count += 1;
            }
        }
    }
    count
}

pub fn main() {
    assert_eq!(SIZE, INPUT.lines().nth(0).unwrap().len() + 1);
    assert_eq!(SIZE, INPUT.lines().count() + 1);

    let start_time = std::time::Instant::now();
    let index = INPUT.chars().position(|x| x == 'S').expect("Could not find start");
    let start_pos = (index % SIZE, index / SIZE);
    let route = walk(start_pos);
    
    let reductions = find_reductions(&route, 2);
    println!("Part 1: {:?}", reductions);

    let reductions = find_reductions(&route, 20);
    println!("Part 2: {:?}", reductions);

    let elapsed = start_time.elapsed().as_micros();
    println!("{}.{:03}ms", elapsed / 1000, elapsed % 1000);
}
