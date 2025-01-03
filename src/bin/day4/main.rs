use regex::Regex;

fn transposed(lines: &Vec<&String>) -> Vec<String> {
    return (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|x| x.chars().nth(i).expect("Unable to transpose; unexpected line length"))
                .fold(String::new(), |s, c| s + &c.to_string())
        })
        .collect();
}

fn diagonals(lines: &Vec<&String>) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    
    let mut d1: Vec<String> = (0..lines.len()).map(|i| {
        (0..i+1).map(|x| {
            let line: &String = &lines.iter().nth(i - x).expect("No such line");
            line.chars().nth(x).expect("Invalid line length")
        }).fold(String::new(), |s, c| s + &c.to_string())
    }).collect();

    let mut d2: Vec<String> = (1..lines.len()).map(|i| {
        (0..i).map(|x| {
            let line: &String = &lines.iter().nth(lines.len() - x - 1).expect("No such line");
            line.chars().nth(lines.len() - i + x).expect("Invalid line length")
        }).fold(String::new(), |s, c| s + &c.to_string())
    }).collect();

    ret.append(&mut d1);
    ret.append(&mut d2);

    return ret;
}

pub fn main() {
    let lines: Vec<String> = include_str!("input.txt")
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let mut counter: i16 = 0;
    let mut counter2: i16 = 0;
    for line in lines.iter() {
        counter += re1.captures_iter(line).count() as i16;
        counter += re2.captures_iter(line).count() as i16;
    }

    for column in transposed(&lines.iter().collect()) {
        counter += re1.captures_iter(column.as_str()).count() as i16;
        counter += re2.captures_iter(column.as_str()).count() as i16;
    }

    for diag in diagonals(&lines.iter().collect()) {
        counter += re1.captures_iter(diag.as_str()).count() as i16;
        counter += re2.captures_iter(diag.as_str()).count() as i16;
    }

    let rev_lines: Vec<&String> = lines.iter().rev().collect();
    for diag2 in diagonals(&rev_lines) {
        counter += re1.captures_iter(diag2.as_str()).count() as i16;
        counter += re2.captures_iter(diag2.as_str()).count() as i16;
    }

    for i in 1..lines.len() - 1 {
        for j in 1..lines.len() - 1 {
            if lines
                .iter()
                .nth(i)
                .expect("No midline")
                .chars()
                .nth(j)
                .expect("Invalid mid line length")
                == 'A'
            {
                let char1 = lines
                    .iter()
                    .nth(i - 1)
                    .expect("No top line")
                    .chars()
                    .nth(j - 1)
                    .expect("Invalid top left");
                let char2 = lines
                    .iter()
                    .nth(i - 1)
                    .expect("No top line")
                    .chars()
                    .nth(j + 1)
                    .expect("Invalid top right");
                let char3 = lines
                    .iter()
                    .nth(i + 1)
                    .expect("No bottom line")
                    .chars()
                    .nth(j - 1)
                    .expect("Invalid bottom left");
                let char4 = lines
                    .iter()
                    .nth(i + 1)
                    .expect("No bottom line")
                    .chars()
                    .nth(j + 1)
                    .expect("Invalid bottom right");

                if char1 == 'M' && char2 == 'M' && char3 == 'S' && char4 == 'S' {
                    counter2 += 1;
                } else if char1 == 'S' && char2 == 'M' && char3 == 'S' && char4 == 'M' {
                    counter2 += 1;
                } else if char1 == 'S' && char2 == 'S' && char3 == 'M' && char4 == 'M' {
                    counter2 += 1;
                } else if char1 == 'M' && char2 == 'S' && char3 == 'M' && char4 == 'S' {
                    counter2 += 1;
                }
            }
        }
    }

    println!("Part 1: {:?}", counter); // 2406
    println!("Part 2: {:?}", counter2); // 1807
}
