use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test2.txt") } else { include_str!("input.txt") };

const LINE_LEN: usize = if TEST { 18 } else { 142 }; // 131 or 11

type Loc = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (-1,0);
const RIGHT: Dir = (0,1);
const DOWN: Dir = (1,0);
const LEFT: Dir = (0,-1);


// fn show(grid: Map) {
//     for line in grid.iter() {
//         println!("{}", line.iter().fold("".to_string(), |o,n| o+&n.to_string()));
//     }
// }

#[derive(PartialEq, Eq)]
pub struct State {
    location: Loc,
    direction: Dir,
    cost: usize,
    history: HashSet<Loc>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost
            .cmp(&other.cost).reverse())
    }
}

fn _unsafe_step(pos: Loc, dir: Dir) -> Loc {
    ((pos.0 as isize + dir.0) as usize, (pos.1 as isize + dir.1) as usize)
}

fn walk(pos: Loc) -> (usize, HashSet<Loc>) {
    let bytes = INPUT.as_bytes();

    let mut best_cost: HashMap<(Loc, Dir), usize> = HashMap::new();
    let mut optimal_cost: usize = usize::MAX;
    let mut optimal_tiles: HashSet<Loc> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::from([State { location: pos, direction: RIGHT, cost: 0, history: HashSet::from([pos]) }]);

    while let Some(state) = heap.pop() {
        for direction in [UP, RIGHT, DOWN, LEFT] {
            let new_pos = _unsafe_step(state.location, direction);
            let tile: u8 = bytes[new_pos.0 * LINE_LEN + new_pos.1];
            let cost: usize = state.cost + if state.direction == direction {1} else {1001};
            
            if cost > optimal_cost {
                continue;
            }

            if tile == b'E' {
                optimal_cost = cost;
                optimal_tiles.extend(state.history.iter());
                optimal_tiles.insert(new_pos);
            } else if tile == b'.' {
                let previous_best: Option<&usize> = best_cost.get(&(new_pos, direction));
                if previous_best.is_none() || &cost <= previous_best.unwrap() {
                    let mut history = state.history.clone();
                    history.insert(new_pos);
                    best_cost.insert((new_pos, direction), cost);
                    heap.push(State {location: new_pos, direction, cost, history});
                }
            }
        }
    }
    return (optimal_cost, optimal_tiles);
}

pub fn main() {
    assert_eq!(LINE_LEN, INPUT.lines().count() + 1);
    assert_eq!(LINE_LEN, INPUT.lines().nth(0).unwrap().len() + 1);

    let index = INPUT.chars().position(|x| x == 'S').expect("Could not find start");
    let pos = (index / LINE_LEN, index % LINE_LEN);

    let (cost, good_tiles) = walk(pos);

    println!("Part 1: {}", cost); // 85396
    println!("Part 2: {}", good_tiles.len()); // 428

}