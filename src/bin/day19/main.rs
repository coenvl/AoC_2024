use std::collections::HashMap;

const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};

fn count_possibilities(
    design: &str,
    patterns: &Vec<&str>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut count: u64 = 0;
    for &prefix in patterns.iter() {
        if design == prefix {
            count += 1;
        } else if design.starts_with(prefix) {
            count += count_possibilities(&design[prefix.len()..], patterns, cache);
        }
    }

    cache.insert(design.to_string(), count);
    count
}

pub fn main() {
    let patterns: Vec<&str> = INPUT
        .lines()
        .nth(0)
        .expect("Unable to read first line")
        .split(", ")
        .collect();

    let designs: Vec<String> = INPUT.lines().skip(2).map(str::to_string).collect();

    let n_possibilities: Vec<u64> = designs
        .iter()
        .map(|x| count_possibilities(&x, &patterns, &mut HashMap::new()))
        .collect();

    let part1: usize = n_possibilities.iter().filter(|&&x| x > 0).count();
    let part2: u64 = n_possibilities.iter().sum();

    println!("Part 1: {}", part1); // 308
    println!("Part 2: {}", part2); // 662_726_441_391_898

    // Part 1: 1223326
    // Part 2: 21070419
}
