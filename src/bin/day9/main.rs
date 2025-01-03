const DATA: &str = include_str!("input.txt");
const EMPTY: isize = -1;

#[derive(Clone)]
struct Block {
    index: usize,
    size: usize,
    free: bool,
}

fn _flatten(blocks: &Vec<Block>) -> Vec<isize> {
    return blocks
        .iter()
        .flat_map(|block| [if block.free { EMPTY } else { block.index as isize }].repeat(block.size))
        .collect();
}

fn _checksum(disk: Vec<isize>) -> usize {
    disk
        .iter()
        .enumerate()
        .map(|(i, &n)| if n == EMPTY { 0 } else { i * n as usize})
        .sum()
}

pub fn main() {
    let mut blocks: Vec<Block> = Vec::new();
    for (i, c) in DATA.char_indices() {
        let n = c.to_digit(10).unwrap() as usize;
        blocks.push(Block {
            index: i / 2,
            size: n,
            free: i % 2 != 0,
        })
    }

    let mut disk: Vec<isize> = _flatten(&blocks);

    for i in 0..disk.len() {
        match disk.get(i) {
            Some(&EMPTY) => {
                disk.swap_remove(i);
                while disk.last().unwrap() == &EMPTY {
                    disk.pop();
                }
            }
            _ => continue,
        }
    }

    let result: usize = _checksum(disk);
    println!("Part 1: {}", result); // 6386640365805

    for j in (0..blocks.len()).rev() {
        if !blocks[j].free {
            for i in 0..j {
                if blocks[i].free && blocks[i].size >= blocks[j].size {
                    blocks[j].free = true; // make the end_block available
                    blocks[i].size -= blocks[j].size;
                    blocks.insert(i, Block {
                        index: blocks[j].index,
                        size: blocks[j].size,
                        free: false
                    }); // insert the moved block
                    break;
                }
            }
        }
    }

    let disk2: Vec<isize> = _flatten(&blocks);
    let result: usize = _checksum(disk2);

    println!("Part 2: {}", result); // 6423258376982
    assert_eq!(result, 6423258376982);
}
