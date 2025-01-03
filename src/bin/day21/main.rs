use std::collections::{HashSet,HashMap};
use std::time::Instant;

const _INPUT: &str = "319A
985A
340A
489A
964A";


const _INPUT2: &str = "029A
980A
179A
456A
379A";

// 789
// 456
// 123
//  0A
fn key_loc(c: char) -> (u8,u8) {
    match c {
        '1' => (0,2),
        '2' => (1,2),
        '3' => (2,2),
        '4' => (0,1),
        '5' => (1,1),
        '6' => (2,1),
        '7' => (0,0),
        '8' => (1,0),
        '9' => (2,0),
        '0' => (1,3),
        'A' => (2,3),
        _ => panic!("No such button") 
    }
}

fn travel_from_to(src: (u8, u8), dst: (u8, u8), empty: (u8, u8), h_first: bool) -> String {
    let mut ret = String::new();
    let mut pos = src.clone();
    while h_first && pos.0 < dst.0 {
        ret.push('>');
        pos.0 += 1;
        if pos == empty { return EMPTY; }
    }
    while h_first && pos.0 > dst.0 {
        ret.push('<');
        pos.0 -= 1;
        if pos == empty { return EMPTY; }
    }
    while pos.1 < dst.1 {
        ret.push('v');
        pos.1 += 1;
        if pos == empty { return EMPTY; }
    }
    while pos.1 > dst.1 {
        ret.push('^');
        pos.1 -= 1;
        if pos == empty { return EMPTY; }
    }
    while pos.0 < dst.0 {
        ret.push('>');
        pos.0 += 1;
        if pos == empty { return EMPTY; }
    }
    while pos.0 > dst.0 {
        ret.push('<');
        pos.0 -= 1;
        if pos == empty { return EMPTY; }
    }
    ret.push('A');
    return ret;
}

const EMPTY: String = String::new();

// 789
// 456
// 123
//  0A
fn solve_keys(code: &str) -> HashSet<String> {
  let mut ret: HashSet<String> = HashSet::new();
  ret.insert(EMPTY);

  let mut pos = key_loc('A'); // A (10)
  for c in code.chars() {
    let dst = key_loc(c);
    let mut ret2 = HashSet::new();
    
    let h_first = travel_from_to(pos, dst, (0,3), true);
    let v_first = travel_from_to(pos, dst, (0,3), false);
    
    for route in ret {
        if h_first != EMPTY {
            ret2.insert(route.clone() + h_first.as_str());
        }
        if v_first != EMPTY {
            ret2.insert(route.clone() + v_first.as_str());
        } 
    }
    
    ret = ret2.clone();
    pos = dst;
  }
  return ret;
}

fn control(pat: &str) -> &str {
    match pat {
    // leaves out the final A for splitting
        "<" => "v<<A>>^A", // 0
        "v<<" => "<vA<AA>>^A", // 1
        "v<" => "<vA<A>>^A", // 2
        "v" => "<vA^>A", // 3
        "" => "A", // 4
        ">>^" => "vAA<^A>A", // 5
        "^" => "<A>A", // 6
        ">" => "vA^A", // 7 // 8
        "<^" => "v<<A>^A>A", // 9
        ">^" => "vA<^A>A", // 10
        "<v" => "v<<A>A^>A", // 11
        ">v" => "vA<A^>A", // 12 doesnt matter?
        "^>" => "<Av>A^A", // 13
        "^<" => "<Av<A>>^A", // 14 doesnt matter?
        "v>" => "<vA>A^A", // 15
        "<<" => "v<<AA>>^A", 
        ">>^^" => "vAA<^AA>A",
        "^^>>" => "<AAv>AA^A",
        "vvv" => "<vAAA^>A",
        "^^^" => "<AAA>A",
        "vv>" => "<vAA^A>A",
        ">vv" => "vA<AA^>A",
        "^<<" => "<Av<AA>>^A",
        "<<^" => "v<<AA>^A>A",
        "^^<<" => "<AAv<AA>>^A",
        ">>vv" => "vAA<AA^>A", // 
        _ => panic!("Pattern not covered: {}", pat)
    }
}

fn get_count(pattern: String, steps: u8, cache: &mut HashMap<(String, u8), usize> ) -> usize {
    if steps == 0 {
        return pattern.len();
    } else if let Some(value) = cache.get(&(pattern.clone(), steps)) {
        return *value;
    }
    let ret = pattern.clone()[0..pattern.len()-1].split('A')
        .map(control)
        .map(|x| get_count(x.to_string(), steps-1, cache))
        .sum();
    cache.insert((pattern.clone(), steps), ret);
    return ret;
}

pub fn main() {
    let start = Instant::now();
    let input = &_INPUT[0..];

    let s1: Vec<HashSet<String>> = input.lines().map(solve_keys).collect();
    let mut cache: HashMap<(String, u8), usize> = HashMap::new();
    
    let l1: Vec<usize> = s1.iter().map(|x| {
        x.iter().map(|y| get_count(y.clone(), 2, &mut cache)).min().unwrap()
    }).collect();
    
    let part1: usize = input.lines().zip(l1.iter()).map(|(line,length)| {
       length * line[..3].parse::<usize>().unwrap()
    }).sum();
    
    println!("Part 1: {}", part1);
    assert_eq!(part1, 215374);
    
    let l2: Vec<usize> = s1.iter().map(|x| {
        x.iter().map(|y| get_count(y.clone(), 25, &mut cache)).min().unwrap()
    }).collect();
    
    let part2: usize = input.lines().zip(l2.iter()).map(|(line,length)| {
       length * line[..3].parse::<usize>().unwrap()
    }).sum();
    
    assert_eq!(part2, 260586897262600);
    
    println!("Part 2: {}", part2);
    println!("Time elapsed: {:?}", start.elapsed());
    
}