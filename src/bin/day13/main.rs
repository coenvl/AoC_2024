use core::str::Lines;

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };

fn parse_line(line: &str, x_pat: &str, y_pat: &str) -> (i64,i64) {
    let start_x = line.find(x_pat).expect("Could not find X") + x_pat.len();
    let end_x = start_x + line[start_x..].find(",").expect("Could not find ,");
    let start_y = end_x + line[end_x..].find(y_pat).expect("Could not find Y") + y_pat.len();
    let x: i64 = line[start_x..end_x].parse().expect("Unable to parse X");
    let y: i64 = line[start_y..].parse().expect("Unable to parse Y");
    (x,y)
}

fn find_solutions(lines: &mut Lines<>, offset: i64) -> i64 {
    let mut count: i64 = 0;
    while let (Some(line1), Some(line2), Some(line3), _) =
                (lines.next(), lines.next(), lines.next(), lines.next()) {

        let a = parse_line(line1, "X+", "Y+");
        let b = parse_line(line2, "X+", "Y+");

        let p_raw = parse_line(line3, "X=", "Y=");
        let p = (offset + p_raw.0, offset + p_raw.1);
        
        let nom = a.1 * p.0 - a.0 * p.1;
        let denom = a.1 * b.0 - a.0 * b.1;
        if nom / denom > 0 && nom % denom == 0 {
            let bi = nom / denom;
            let ai = (p.1 - bi * b.1) / a.1;
            if ai > 0 && (p.1 - bi * b.1) % a.1 == 0 {
                count += 3 * ai + bi;
            }
        }
    }
    return count;
}

pub fn main() {
    let mut lines = INPUT.lines();
    
    let part1 = find_solutions(&mut lines.clone(), 0);
    println!("Part 1: {}", part1);

    let part2 = find_solutions(&mut lines, 10000000000000);
    println!("Part 2: {}", part2);

    assert_eq!(part1, 31552);
    assert_eq!(part2, 95273925552482);
    
}