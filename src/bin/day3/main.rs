use regex::Regex;

pub fn main() {
    let lines = include_str!("input.txt");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enable: i32 = 1;
    let mut counter1 = 0;
    let mut counter2 = 0;

    for cap in re.captures_iter(lines) {
        if cap.get(1).is_none() {
            enable = (cap[0].len() == 4) as i32;
        } else if cap.iter().count() == 3 {
            let nums: Vec<i32> = cap
                .iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse().unwrap())
                .collect();
            counter1 += nums[0] * nums[1];
            counter2 += enable * nums[0] * nums[1];
        }
    }

    println!("Part 1: {}", counter1); // 155955228
    println!("Part 2: {}", counter2); // 100189366
}
