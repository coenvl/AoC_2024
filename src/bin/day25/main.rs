const INPUT: &str = include_str!("input.txt");

fn count_cols(txt: &&str) -> [u8; 5] {
    let mut ret: [u8; 5] = [0; 5];
    txt.chars()
        .enumerate()
        .filter(|(_i, c)| *c == '#')
        .map(|(i,_c)| ret[i%6] += 1)
        .last();
    ret
}

fn main() {
    let locks: Vec<&str> = INPUT
        .split("\n\n")
        .filter(|x| x.chars().nth(0).unwrap() == '#')
        .collect();
        
    let keys: Vec<&str> = INPUT
        .split("\n\n")
        .filter(|x| x.chars().nth(0).unwrap() == '.')
        .collect();
    
    let num_locks: Vec<[u8;5]> = locks
        .iter()
        .map(count_cols)
        .collect();
        
    let num_keys: Vec<[u8;5]> = keys
        .iter()
        .map(count_cols)
        .collect();
        
    let mut counter = 0;
    for k in num_keys.iter() {
        for l in num_locks.iter() {
            
            let isvalid = k.iter()
                .zip(l.iter())
                .any(|(a,b)| a+b>7);
            //println!("{k:?} + {l:?} ({isvalid})");
            if !isvalid {
                counter += 1;
            }
        }
    }
    
    println!("Part 1: {counter}");
}