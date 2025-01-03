use std::collections::HashSet;

const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};
const LINE_LEN: usize = if TEST { 11 } else { 141 };
const NUM_LINES: usize = LINE_LEN;

const TOP: u8 = 0;
const LEFT: u8 = 1;
const RIGHT: u8 = 2;
const BOTTOM: u8 = 3;

fn follow_region(input: &[u8], start: usize, region: &mut HashSet<usize>) {
    let mut stack = vec![start];
    while let Some(i) = stack.pop() {
        if region.insert(i) {
            let c = input[i];
            if i % LINE_LEN > 0 && input[i - 1] == c {
                stack.push(i - 1);
            }
            if i % LINE_LEN < LINE_LEN - 1 && input[i + 1] == c {
                stack.push(i + 1);
            }
            if i >= LINE_LEN && input[i - LINE_LEN] == c {
                stack.push(i - LINE_LEN);
            }
            if i + LINE_LEN < input.len() && input[i + LINE_LEN] == c {
                stack.push(i + LINE_LEN);
            }
        }
    }
}

fn get_perimeter(region: &HashSet<usize>) -> Vec<(usize, usize, u8)> {
    let mut fences = Vec::new();
    for &i in region {
        let x = i / LINE_LEN;
        let y = i % LINE_LEN;

        if x == 0 || !region.contains(&(i - LINE_LEN)) {
            fences.push((i, i + 1, TOP));
        }
        if y == 0 || !region.contains(&(i - 1)) {
            fences.push((i, i + LINE_LEN, LEFT));
        }
        if y == LINE_LEN - 1 || !region.contains(&(i + 1)) {
            fences.push((i + 1, i + 1 + LINE_LEN, BOTTOM));
        }
        if x == NUM_LINES - 1 || !region.contains(&(i + LINE_LEN)) {
            fences.push((i + LINE_LEN, i + LINE_LEN + 1, RIGHT));
        }
    }
    fences.sort();
    fences
}

fn count_parts(fences: Vec<&(usize, usize, u8)>) -> usize {
    let mut parts: Vec<(usize, usize, u8)> = Vec::new();
    for &fence in fences {
        if let Some(i) = parts.iter().position(|&(_, end, _)| end == fence.0) {
            parts[i].1 = fence.1;
        } else {
            parts.push(fence);
        }
    }
    parts.len()
}

fn count_edges(perimeter: &Vec<(usize, usize, u8)>) -> usize {
    [TOP, BOTTOM, LEFT, RIGHT]
        .iter()
        .map(|&d| {
            let h_parts: Vec<&(usize, usize, u8)> =
                perimeter.iter().filter(|&&p| p.2 == d).collect();
            h_parts
        })
        .map(count_parts)
        .sum()
}

pub fn main() {
    assert_eq!(LINE_LEN, INPUT.lines().nth(0).unwrap().len() + 1);
    assert_eq!(NUM_LINES, INPUT.lines().count() + 1);

    let start_time = std::time::Instant::now();

    let input_bytes = INPUT.as_bytes();

    let mut mapped = HashSet::new();

    let mut part1: usize = 0;
    let mut part2: usize = 0;

    for i in 0..input_bytes.len() {
        if input_bytes[i] != b'\n' && !mapped.contains(&i) {
            let mut region = HashSet::new();
            follow_region(input_bytes, i, &mut region);
            let perimeter = get_perimeter(&region);
            let n_edges = count_edges(&perimeter);
            part1 += region.len() * perimeter.len();
            part2 += region.len() * n_edges;
            mapped.extend(region);
        }
    }

    let elapsed = start_time.elapsed().as_micros();

    assert_eq!(part1, if TEST { 1930 } else { 1473408 });
    assert_eq!(part2, if TEST { 1206 } else { 886364 });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!("{}.{:03}ms", elapsed / 1000, elapsed % 1000);
}
