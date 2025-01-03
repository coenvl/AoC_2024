use std::usize;

const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
// const STEPS: isize = 100;

fn parse() -> (Vec<(usize, usize)>, Vec<(isize, isize)>) {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let mut velocities: Vec<(isize, isize)> = Vec::new();

    for line in INPUT.lines() {
        let words: Vec<&str> = line.split(" v=").collect();
        let pos: Vec<usize> = words[0][2..].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let speed: Vec<isize> = words[1].split(",").map(|x| x.parse::<isize>().unwrap()).collect();
        positions.push((pos[0], pos[1]));
        velocities.push((speed[0], speed[1]));
    }
    (positions, velocities)
}

fn safety_code(positions: &Vec<(usize, usize)>, velocities: &Vec<(isize, isize)>, steps: usize) -> usize {
    let mut q: [usize; 4] = [0; 4];
    positions.iter().zip(velocities.iter()).map(|(&pos, &vel)| {
        let x = (pos.0 as isize + vel.0 * steps as isize).rem_euclid(WIDTH as isize) as usize;
        let y = (pos.1 as isize + vel.1 * steps as isize).rem_euclid(HEIGHT as isize) as usize;

        if x < WIDTH / 2 && y < HEIGHT / 2 {
            q[0] += 1;
        }
        if x > (WIDTH) / 2 && y < HEIGHT / 2 {
            q[1] += 1;
        }
        if x < WIDTH / 2 && y > (HEIGHT) / 2 {
            q[2] += 1;
        }
        if x > (WIDTH )/ 2 && y > (HEIGHT) / 2 {
            q[3] += 1;
        }
    }).last();
    q[0]*q[1]*q[2]*q[3]
}

fn _display(positions: &Vec<(usize, usize)>, velocities: &Vec<(isize, isize)>, steps: usize) {
    let mut grid: [[char; WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];
    positions.iter().zip(velocities.iter()).map(|(&pos, &vel)| {
        let x = (pos.0 as isize + vel.0 * steps as isize).rem_euclid(WIDTH as isize) as usize;
        let y = (pos.1 as isize + vel.1 * steps as isize).rem_euclid(HEIGHT as isize) as usize;
        grid[y][x] = '#';

    }).last();

    for line in grid.iter() {
        println!("{}", line.iter().fold("".to_string(), |o,n| o+&n.to_string()));
    }
}

fn main() {
    let (positions, velocities) = parse();

    let cost = safety_code(&positions, &velocities, 100);
    println!("Part 1: {}", cost); // 85396

    let part2 = (1..10_000)
        .map(|x| (safety_code(&positions, &velocities, x), x))
        .min().unwrap().1;

    //_display(&positions, &velocities, part2);

    println!("Part 2: {}", part2);
}