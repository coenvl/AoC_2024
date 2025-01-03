fn line_to_vec(line: &str) -> Vec<i16> {
    return line.split(' ').map(|y| y.parse().expect("Unable to parse line")).collect();
}

fn is_safe(levels: &Vec<i16>) -> bool {
    let diffs: Vec<i16> = levels
        .iter()
        .zip(levels.iter().skip(1))
        .map(|(x, y)| x - y)
        .collect();
    return (diffs.iter().all(|&x| x > 0) || diffs.iter().all(|&x| x < 0))
        && diffs.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3);
}

fn except(levels: &Vec<i16>, index: usize) -> Vec<i16> {
    let mut subset = levels.clone();
    subset.remove(index);
    return subset
}

fn is_skip_safe(levels: &Vec<i16>) -> bool {
    return (0..levels.len())
        .map(|i| except(levels, i))
        .any(|x| is_safe(&x));
}

pub fn main() {
    let lines = include_str!("input.txt").lines();
    let reports: Vec<Vec<i16>> = lines.map(line_to_vec).collect();

    let counter1 = reports.iter().filter(|&r| is_safe(r)).count();
    let counter2 = reports.iter().filter(|&r| is_skip_safe(r)).count();

    println!("Part 1: {}", counter1);
    println!("Part 2: {}", counter2);
}
