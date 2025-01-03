// use std::cmp::{min,max};
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const LINE_LEN: isize = 51; // 13 or 51
const NUM_LINES: isize = 50; // 12 or 50

fn _projections(c: &char, include_harmonics: bool) -> HashSet<(isize, isize)> {
    let m: Vec<_> = INPUT
        .match_indices(*c)
        .map(|m| (m.0 as isize / LINE_LEN, m.0 as isize % LINE_LEN))
        .collect(); // Locations of the antennae

    return m
        .iter()
        .flat_map(|&pa| {
            return m
                .iter()
                .filter(move |&pb| pa != *pb)
                .flat_map(move |&pb| {
                    let d = (pa.0 - pb.0, pa.1 - pb.1);
                    if !include_harmonics {
                        vec![(pa.0 + d.0, pa.1 + d.1)]
                    } else {
                        let mut p: Vec<(isize,isize)> = vec![pa];
                        let mut c = pa.clone();
                        while c.0 >= 0 && c.1 >= 0 && c.0 < NUM_LINES && c.1 < LINE_LEN - 1 {
                            p.push(c);
                            c.0 += d.0;
                            c.1 += d.1;
                        }
                        p
                    }
                });
        })
        .collect();
}

pub fn main() {
    assert_eq!(LINE_LEN - 1, INPUT.lines().nth(0).unwrap().len() as isize);
    assert_eq!(NUM_LINES, INPUT.lines().count() as isize);

    let chars: HashSet<char> = INPUT.chars().filter(|&x| x != '\n' && x != '.').collect();

    let projections: HashSet<(isize, isize)> = chars
        .iter()
        .flat_map(|x| _projections(x, false))
        .filter(|x| x.0 >= 0 && x.1 >= 0 && x.0 < NUM_LINES && x.1 < LINE_LEN - 1)
        .collect();
    println!("{:?}", projections.len()); // 367

    let projections: HashSet<(isize, isize)> = chars
        .iter()
        .flat_map(|x| _projections(x, true))
        .collect();
    println!("{:?}", projections.len()); // 1285

}
