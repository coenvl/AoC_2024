use std::collections::VecDeque;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn operate(program: &Vec<usize>, registry: &mut [usize; 3]) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    let mut i: usize = 0;

    while i < (program.len()) {
        let lit_operand: usize = program[i + 1];
        let combo_operand: usize = match lit_operand {
            4 => registry[A],
            5 => registry[B],
            6 => registry[C],
            _ => lit_operand,
        };

        match program[i] {
            0 => registry[A] = registry[A] / 2_usize.pow(combo_operand as u32) as usize,
            1 => registry[B] = registry[B] ^ lit_operand,
            2 => registry[B] = combo_operand % 8,
            3 => {
                if registry[A] != 0 {
                    i = lit_operand;
                    continue;
                }
            }
            4 => registry[B] = registry[C] ^ registry[B],
            5 => output.push(combo_operand % 8),
            6 => registry[B] = registry[A] / 2_usize.pow(combo_operand as u32),
            7 => registry[C] = registry[A] / 2_usize.pow(combo_operand as u32),
            _ => panic!("Invalid instruction {}", program[i]),
        }
        i += 2
    }

    return output;
}

fn read_bit_code(bit_code: &Vec<u8>, base: u32) -> usize {
    bit_code
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| 8_usize.pow(base + i as u32) * x as usize)
        .sum()
}

fn find_last_bits(program: &Vec<usize>, init: usize) -> Option<usize> {
    (0..64).find_map(|j| {
        if &operate(&program, &mut [init + j, 0, 0]) == program {
            Some(init + j)
        } else {
            None
        }
    })
}

pub fn main() {
    let program: Vec<usize> = vec![2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0];
    // let program: Vec<usize> = vec![2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0];
    let lower_bound = 8_usize.pow(15);

    let mut registry: [usize; 3] = [61156655, 0, 0];
    let output =  operate(&program, &mut registry)
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("Part1: {output}");

    let start_time = std::time::Instant::now();
    let mut potentials: VecDeque<Vec<u8>> = VecDeque::from([vec![]]);

    'outer: while let Some(bit_code) = potentials.pop_front() {
        let init: usize = read_bit_code(&bit_code, 1);

        for i in 0..8 {
            let reg_a = lower_bound + init + i as usize;
            let mut registry = [reg_a, 0, 0];

            if operate(&program, &mut registry)[0] == program[15 - bit_code.len()] {
                let mut new_bit_code = bit_code.to_owned();
                new_bit_code.push(i);
                if bit_code.len() > 12 {
                    let init2 = (init + i as usize) << 6;
                    if let Some(solution) = find_last_bits(&program, init2) {
                        println!("Part2: {}", solution);
                        break 'outer;
                    }
                } else {
                    potentials.push_back(new_bit_code);
                }
            }
        }
    }

    let elapsed = start_time.elapsed().as_micros();
    println!("{}.{:03}ms", elapsed / 1000, elapsed % 1000);

    // println!("Part2: {:?}", solutions.iter().min().expect("No solutions found"));

    // 105734774294938 is the right answer
}
