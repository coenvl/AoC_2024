use memoize::memoize;

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };

fn split_even(num: usize) -> Option<(usize, usize)> {
    let n = num.to_string().len();
    if n % 2 == 0 {
        let a = 10_usize.pow((n / 2) as u32);
        Some((num / a, num % a))
    } else {
        None
    }
}

#[memoize]
fn count(num: usize, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    } else if num == 0 {
        return count(1, blinks - 1);
    } else if let Some(parts) = split_even(num) {
        return count(parts.0, blinks - 1) + count(parts.1, blinks -1);
    }
    count(num * 2024, blinks - 1)
}

pub fn main() {
    let nums: Vec<usize> = INPUT.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let total: usize = nums.iter().map(|&x| count(x, 25)).sum();
    println!("Part 1: {}", total);
    
    let total2: usize = nums.iter().map(|&x| count(x, 75)).sum();
    println!("Part 2: {}", total2);
}
