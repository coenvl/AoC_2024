use std::collections::HashSet;
const TEST: bool = false;
const INPUT: &str = if TEST { include_str!("test.txt") } else { include_str!("input.txt") };

type Coords = (i16, i16);

fn parse_input(input: &str) -> (Coords, HashSet<Coords>, HashSet<Coords>, Vec<char>) {
    let line_len = input.chars().position(|c| c == '\n').unwrap() as i16 + 1;
    let p: i16 = input.chars().position(|c| c == '@').expect("Robot🤖 not found!") as i16;
    let pos: Coords = (p / line_len, p % line_len);
    let walls: HashSet<Coords> = input.chars().enumerate().filter(|&(_,c)| c == '#').map(|(i,_)| (i as i16 % line_len, i as i16 / line_len)).collect();
    let boxes: HashSet<Coords> = input.chars().enumerate().filter(|&(_,c)| c == 'O').map(|(i,_)| (i as i16 % line_len, i as i16 / line_len)).collect();
    let moves: Vec<char> = input.split("\n\n").nth(1).unwrap().chars().collect();
    return (pos, walls, boxes, moves);
}

fn step(pos: &mut Coords, walls: &HashSet<Coords>, boxes: &mut HashSet<Coords>, x: i16, y: i16) {
    let dst = (pos.0 + x, pos.1 + y);
    if boxes.contains(&dst) {
        if !push(boxes, walls, &dst, x, y) {
            return
        }
    }
    if walls.contains(&dst) {
        return;
    }
    pos.0 += x;
    pos.1 += y;
}

fn push(boxes: &mut HashSet<Coords>, walls: &HashSet<Coords>, src: &Coords, x: i16, y: i16) -> bool {
    let dst = (src.0 + x, src.1 + y);
    if walls.contains(&dst) {
        return false;
    }
    if boxes.contains(&dst) {
        let possible = push(boxes, walls, &dst, x, y);
        if !possible {
            return false;
        }
    }
    boxes.remove(src);
    boxes.insert(dst);
    return true;
}

fn main() {
  let (mut pos, walls, mut boxes, moves) = parse_input(INPUT);
//   println!("{:?}", pos);
//   println!("{:?}", walls);
//   println!("{:?}", boxes);
//   println!("{:?}", moves);
  
  for d in moves {
      match d {
          '<' => step(&mut pos, &walls, &mut boxes, -1, 0),
          '^' => step(&mut pos, &walls, &mut boxes, 0, -1),
          '>' => step(&mut pos, &walls, &mut boxes, 1, 0),
          'v' => step(&mut pos, &walls, &mut boxes, 0, 1),
          _ => continue
      }
    //   println!("------------");
    //   println!("{:?}", pos);
    //   println!("{:?}", boxes);
   
  }
  let sum: usize = boxes.iter().map(|b| 100 * b.1 as usize + b.0 as usize).sum();
  println!("{}", sum);
   
}