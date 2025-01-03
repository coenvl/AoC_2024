pub fn main() {
    let lines = include_str!("input.txt").lines();

    let mut set_a: Vec<i32> = lines.clone().map(|x| x[..5].parse().unwrap()).collect();
    let mut set_b: Vec<i32> = lines.map(|x| x[8..].parse().unwrap()).collect();

    set_a.sort();
    set_b.sort();

    let z : Vec<i32> = set_a.iter().zip(set_b.iter()).map(|(x,y)| (x-y).abs() ).collect();
    let total: i32 = z.iter().sum();
    
    println!("Part 1: {}", total);

    let z2: Vec<i32> = set_a.iter().map(|x| x * set_b.iter().filter(|&y| x == y).count() as i32).collect();
    let total2: i32 = z2.iter().sum();

    println!("Part 2: {}", total2);

    // Part 1: 1223326
    // Part 2: 21070419
}