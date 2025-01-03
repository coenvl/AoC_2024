use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};
const SIZE: u16 = if TEST { 6 } else { 70 };
const MAX_TIME: u16 = if TEST { 12 } else { 1024 };

type Loc = (u16, u16);
type Dir = (i16, i16);

const UP: Dir = (-1, 0);
const RIGHT: Dir = (0, 1);
const DOWN: Dir = (1, 0);
const LEFT: Dir = (0, -1);

#[derive(PartialEq, Eq, Debug)]
pub struct State {
    cost: u16,
    location: Loc,
    steps: u16,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

fn step(pos: Loc, dir: Dir) -> Option<Loc> {
    match dir {
        UP if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        DOWN if pos.1 < SIZE => Some((pos.0, pos.1 + 1)),
        LEFT if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        RIGHT if pos.0 < SIZE => Some((pos.0 + 1, pos.1)),
        _ => None,
    }
}

fn walk(bytes: &HashMap<Loc, u16>, max_time: u16) -> Option<u16> {
    let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
        cost: SIZE + SIZE,
        location: (0, 0),
        steps: 0,
    }]);
    let mut best_cost: HashMap<Loc, u16> = HashMap::new();

    while let Some(state) = heap.pop() {
        for direction in [UP, RIGHT, DOWN, LEFT] {
            if let Some(location) = step(state.location, direction) {
                if let Some(block) = bytes.get(&location) {
                    if block < &max_time {
                        continue; // Blocked
                    }
                }

                let steps = state.steps + 1;
                if location == (SIZE, SIZE) {
                    return Some(steps); // Target
                }

                let previous_best = best_cost.get(&location);
                if previous_best.is_none() || steps < *previous_best.unwrap() {
                    best_cost.insert(location, steps);
                    let cost = steps + (SIZE - location.0) + (SIZE - location.1);
                    heap.push(State {
                        cost,
                        location,
                        steps,
                    });
                }
            }
        }
    }
    None
}

pub fn main() {
    let bytes: HashMap<Loc, u16> = INPUT
        .split("\n")
        .filter(|x| !x.is_empty())
        .enumerate()
        .map(|(i, x)| {
            let nums: Vec<u16> = x.split(",").map(|n| n.parse().unwrap()).collect();
            ((nums[0], nums[1]), i as u16)
        })
        .collect();

    let part1 = walk(&bytes, MAX_TIME);
    println!("Part 1: {}", part1.expect("No solution!")); // 288

    let mut lo = MAX_TIME;
    let mut hi = bytes.len() as u16;
    let mut i = lo;
    while lo < hi - 1 {
        if let Some(_solution) = walk(&bytes, i) {
            lo = i;
            i = (i + hi) / 2;
        } else {
            hi = i;
            i = (i + lo) / 2;
        }
    }

    let part2 = bytes.iter().find(|(_k,&v)| v == i).unwrap().0;
    println!("Part 2: {},{}", part2.0, part2.1);

}
