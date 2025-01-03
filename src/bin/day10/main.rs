use std::collections::HashSet;

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };
const LINE_LEN: usize = if TEST { 7 } else { 61 }; // 131 or 11
const NUM_LINES: usize = LINE_LEN;

const UP: isize = - (LINE_LEN as isize);
const RIGHT: isize = 1;
const DOWN: isize = LINE_LEN as isize;
const LEFT: isize = -1;

fn about_to_exit(pos: usize, dir: isize) -> bool {
    match dir {
        UP => pos < LINE_LEN,
        DOWN => pos >= ((LINE_LEN-1) * (NUM_LINES-1)) - 1,
        LEFT => pos % LINE_LEN <= 0,
        RIGHT => pos % LINE_LEN >= (LINE_LEN -2),
        _ => false
    }
}

fn count_trails(start: &usize) -> (usize, usize) {
    let mut ends: HashSet<usize> = HashSet::new();
    let mut next_heads: Vec<usize> = Vec::new();
    let mut heads: Vec<usize> = vec![*start];
    let mut count: usize = 0;

    for next_num in ['1', '2', '3', '4','5','6','7','8', '9'] {
        heads.append(&mut next_heads);
        while !heads.is_empty() {
            match heads.pop() {
                Some(h) => {
                    for dir in [UP, RIGHT, DOWN, LEFT] {
                        if about_to_exit(h, dir) {
                            continue;
                        }
                        let next_pos: usize = (h as isize + dir).try_into().unwrap();
                        match INPUT.chars().nth(next_pos) {
                            Some(num) => {
                                if num == next_num && num == '9' {
                                    ends.insert(next_pos);
                                    count += 1;
                                } else if num == next_num {
                                    next_heads.push(next_pos);
                                }
                            },
                            None => continue
                        }
                    }
                },
                None => {}
            }
        }
    }
    return (ends.len(), count);
}

pub fn main() {
    assert_eq!(LINE_LEN, INPUT.lines().nth(0).unwrap().len() + 1);
    assert_eq!(NUM_LINES, INPUT.lines().count() + 1);

    let starts: Vec<usize> = INPUT.match_indices('0').map(|m| m.0).collect();
    let counts: Vec<(usize, usize)> = starts.iter().map(count_trails).collect();
   
    let part1: usize = counts.iter().map(|x| x.0).sum();
    let part2: usize = counts.iter().map(|x| x.1).sum();
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    
    assert_eq!(part1, if TEST {36} else {811});
    assert_eq!(part2, if TEST {81} else {1794});
}