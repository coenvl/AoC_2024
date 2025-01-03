use std::collections::{HashMap, HashSet};

// const DATA: &str = include_str!("test.txt");
// const LINE_LEN: i16 = 11; // 131 or 11
// const START_POS: i16 = 6 * LINE_LEN + 4; // (84,89) or (6,4)

const DATA: &str = include_str!("input.txt");
const LINE_LEN: i16 = 131; // 131 or 11
const START_POS: i16 = 84 * LINE_LEN + 89;

const UP: i16 = -LINE_LEN;
const RIGHT: i16 = 1;
const DOWN: i16 = LINE_LEN;
const LEFT: i16 = -1;

fn gather_obstacles() -> HashSet<i16> {
    DATA.char_indices()
            .filter(|&ci| ci.1 == '#')
            .map(|ci| ci.0 as i16)
            .collect()
}

fn obstacle_in_sight(pos: i16, dir: i16, obstacle_pos: i16) -> bool {
    match dir {
        UP => (obstacle_pos % LINE_LEN) == (pos % LINE_LEN) && (pos / LINE_LEN) > (obstacle_pos / LINE_LEN),
        DOWN => (obstacle_pos % LINE_LEN) == (pos % LINE_LEN) && (pos / LINE_LEN) < (obstacle_pos / LINE_LEN),
        LEFT => (obstacle_pos / LINE_LEN) == (pos / LINE_LEN) &&  (pos % LINE_LEN) > (obstacle_pos % LINE_LEN),
        RIGHT => (obstacle_pos / LINE_LEN) == (pos / LINE_LEN) && (pos % LINE_LEN) < (obstacle_pos % LINE_LEN),
        _ => false
    }
}

// fn about_to_exit(pos: i16, dir: i16) -> bool {
//     match dir {
//         UP => pos < LINE_LEN,
//         DOWN => pos > LINE_LEN * LINE_LEN,
//         LEFT => pos % LINE_LEN <= 0,
//         RIGHT => pos % LINE_LEN >= LINE_LEN - 1,
//         _ => false
//     }
// }

fn about_to_exit(pos: i16, dir: i16) -> bool {
    match dir {
        UP => pos < LINE_LEN,
        DOWN => pos >= ((LINE_LEN-1) * (LINE_LEN-1)) - 1,
        LEFT => pos % LINE_LEN <= 0,
        RIGHT => pos % LINE_LEN >= (LINE_LEN -2),
        _ => false
    }
}

fn follow_path(
    obstacles: &HashSet<i16>,
    obstacle_pos: i16,
    jumps: &mut HashMap<(i16, i16), i16>,
) -> (bool, HashSet<(i16, i16)>) {
    let mut pos = START_POS;
    let mut history: HashSet<(i16, i16)> = HashSet::new();

    loop {
        for dir in [UP, RIGHT, DOWN, LEFT] {
            let obstacle_line = obstacle_in_sight(pos, dir, obstacle_pos);
            if !obstacle_line {
                if let Some(&new_pos) = jumps.get(&(pos, dir)) {
                    // We have followed this "clear" line before, follow the jump
                    pos = new_pos;
                    if history.contains(&(pos, dir)) {
                        return (true, history);
                    }
                    history.insert((pos, dir));
                    continue;
                }
            }
            
            // We have not been here before, walk until we find an obstacle
            let dir_start_pos = pos;
            loop {
                if !history.insert((pos, dir)) {
                    // We looped, return
                    return (true, history);
                } else if about_to_exit(pos, dir) {
                    // We exited the field, return
                    return (false, history);
                }

                // Update position
                let next_pos = pos + dir;
                if next_pos == obstacle_pos || obstacles.contains(&next_pos) {
                    break;
                }
                pos = next_pos;
            }

            if !obstacle_line {
                // We found the end of a "clear" line, update jump cache
                jumps.insert((dir_start_pos, dir), pos);
            }
        }
    }
}

pub fn main() {
    // assert!(DATA.chars().nth(START_POS as usize).unwrap() == '^');
    assert_eq!(LINE_LEN as usize, DATA.lines().nth(0).unwrap().len() + 1);
    assert_eq!(LINE_LEN as usize, DATA.lines().count() + 1);

    let obstacles = gather_obstacles();
    let mut jumps: HashMap<(i16, i16), i16> = HashMap::new(); // act as a cache

    let (_loops, history) = follow_path(&obstacles, i16::MAX, &mut jumps);
    let visited: HashSet<i16> = HashSet::from_iter(history.iter().map(|x| x.0));
    println!("Part 1: {}", visited.len()); // 4982

    let counter: usize = visited
        .iter()
        .filter(|&&o| follow_path(&obstacles, o, &mut jumps).0)
        .count();
    println!("Part 2: {}", counter); // 1663
}
