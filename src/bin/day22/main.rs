use std::collections::HashMap;

use rayon::prelude::*;

const TEST: bool = false;
const INPUT: &str = if TEST {
    include_str!("test.txt")
} else {
    include_str!("input.txt")
};

fn secrets(start: usize, num: usize) -> Vec<usize> {
    let mut x = start;
    let mut ret: Vec<usize> = Vec::with_capacity(num);
    for _i in 0..num {
        x ^= (x << 6) & 0xffffff;
        x ^= (x >> 5) & 0xffffff;
        x ^= (x << 11) & 0xffffff;
        ret.push(x & 0xffffff);
    }
    ret
}

fn main() {
    let all_secrets: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|x| x.parse().unwrap())
        .map(|x| secrets(x, 2000))
        .collect();
    
    let part1: usize = all_secrets.iter().map(|x| x.last().unwrap()).sum();
    println!("Part 1: {part1}");

    let all_caches: &Vec<HashMap<(i8, i8, i8, i8), usize>> = &all_secrets
        .iter()
        .map(|secrets| {
            let mut cache: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();
            let prices: Vec<u8> = secrets.iter().map(|x| (x % 10) as u8).collect();
            let changes: Vec<i8> = prices
                .iter()
                .zip(prices.iter().skip(1))
                .map(|(&a, &b)| b as i8 - a as i8)
                .collect();
            for i in 3..changes.len() {
                let key: (i8, i8, i8, i8) =
                    (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
                if !cache.contains_key(&key) {
                    cache.insert(key, prices[i + 1] as usize);
                }
            }
            cache
        })
        .collect();

    fn foo(n: i8) -> bool {
        n >= -9 && n < 10
    }

    // Parallelize the computation
    let max= (-9..=9)
        .into_par_iter()
        .map(|a| {
            (-9..=9)
                .filter(|&b| foo(a+b))
                .flat_map(|b| {
                    (-9..=9)
                    .filter(move |&c| foo(b+c) && foo(a+b+c))
                    .flat_map(move |c| {
                        (-9..=9)
                            .filter(move |&d| foo(c+d) && foo(b+c+d) && foo(a+b+c+d))
                            .map(move |d| {
                                all_caches
                                    .iter()
                                    .filter_map(|cache| cache.get(&(a, b, c, d)))
                                    .sum()
                            })
                        })
                    })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0);

    println!("Part 2: {max}");
}
