use std::cmp::Ordering;

pub fn main() {
    let input = include_str!("input.txt");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let orderings: Vec<Vec<i16>> = parts[0]
        .split("\n")
        .map(|x| x.split("|").map(|x| x.parse().unwrap()).collect())
        .collect();

    let pages: Vec<Vec<i16>> = parts[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut counter = 0;
    let mut counter2 = 0;

    for page in pages {
        let mut page_ordered = page.clone();
        page_ordered.sort_by(|a,b| {
            let a_before_b = orderings.iter().find(|o| &o[0] == a && &o[1] == b);
            if a_before_b.is_some() {
                return Ordering::Less;
            }
            let b_before_a = orderings.iter().find(|o| &o[0] == b && &o[1] == a);
            if b_before_a.is_some() {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        });

        if page == page_ordered {
            counter += page_ordered[page_ordered.len() / 2];
        } else {
            counter2 += page_ordered[page_ordered.len() / 2];
        }
    }

    println!("Part 1: {}", counter);
    println!("Part 2: {}", counter2); // 4743 is too high

}
