const INPUT: &str=include_str!("input.txt");

fn can_make(line: &str, part2: bool) -> i64 {
    let parts: Vec<&str> = line.split(" ").collect();
    let result: i64 = parts
        .iter()
        .nth(0)
        .unwrap()
        .replace(":", "")
        .parse()
        .unwrap();
    let nums: Vec<i64> = parts.iter().skip(1).map(|x| x.parse().unwrap()).collect();

    let mut options: Vec<i64> = Vec::new();
    options.push(0);

    for p in nums {
        let old = options.clone();
        options.clear();
        for o in old {
            let a1 = o+p;
            if a1 <= result {
                options.push(a1);
            }
            let a2 = o*p;
            if a2 <= result {
                options.push(a2);
            }
            if part2 {
                let a3 = concat(o,p);
                if a3 <= result {
                    options.push(a3);
                }
            }
        }
    }

    if options.contains(&result) {
        return result;
    } else {
        return 0;
    }
}

fn concat(a:i64, b: i64) -> i64 {
    let c: u32 = (b as f32 + 1_f32).log10().ceil() as u32;
    let d: i64 = a as i64 * 10_i64.pow(c) as i64;
    return d + b;
}

fn main() {
    let count: i64 = INPUT.lines().map(|s| can_make(s, false)).sum();
    println!("Part 1: {}", count);

    let count: i64 = INPUT.lines().map(|s| can_make(s, true)).sum();
    println!("Part 2: {}", count);
}