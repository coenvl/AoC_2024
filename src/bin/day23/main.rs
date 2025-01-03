use std::collections::{HashMap, HashSet};

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };

pub fn main() {

    let edges: HashSet<(&str, &str)> = INPUT.lines().map(|x| {
        let parts: Vec<&str> = x.split("-").collect();
        (parts[0], parts[1])
    }).collect();

    let mut routes: HashMap<&str, HashSet<&str>> = HashMap::new();

    for e in edges.iter() {
        if let Some(set) = routes.get_mut(e.0) {
            set.insert(e.1);
        } else {
            routes.insert(e.0, HashSet::from([e.1]));
        }
        if let Some(set) = routes.get_mut(e.1) {
            set.insert(e.0);
        } else {
            routes.insert(e.1, HashSet::from([e.0]));
        }
    }

    // println!("{routes:?}");

    let mut trips: HashSet<Vec<&str>> = HashSet::new();
    for a in routes.keys() {
        if a.starts_with("t") {
            if let Some(dsts) = routes.get(a) {
                for b in dsts.iter() {
                    for c in dsts.iter() {
                        if routes.get(b).unwrap().contains(c) {
                            let mut trip = vec![*a,*b,*c];
                            trip.sort();
                            trips.insert(trip);
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {:?}", trips.len());

    let mut clusters: HashSet<Vec<&str>> = HashSet::new();
    for a in routes.keys() {
        if clusters.iter().any(|x| x.contains(a)) {
            continue;
        }
        let mut a_cluster: Vec<&str> = vec![a];
        let mut stack: Vec<&&str> = a_cluster.iter().flat_map(|&n| routes.get(n).unwrap()).collect();
        while let Some(&neighbor) = stack.pop() {
            if a_cluster.contains(&neighbor) {
                continue;
            }
            if a_cluster.iter().all(|x| routes.get(neighbor).unwrap().contains(x)) {
                a_cluster.push(neighbor);
            }
        }
        a_cluster.sort();
        clusters.insert(a_cluster);
    }
        
    let largest_cluster = clusters.iter().map(|x| (x.len(), x)).max().unwrap();
    let code = largest_cluster.1.join(",");
    println!("Part 2: {code}");
    
}