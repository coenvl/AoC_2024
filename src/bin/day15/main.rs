use std::collections::HashSet;

const _INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

const _INPUT2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const _INPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

const INPUT: &str = include_str!("input.txt");

type Coords = (i16, i16);

fn parse_input(input: &str) -> (Coords, HashSet<Coords>, HashSet<Coords>, Vec<char>) {
    let line_len = input.chars().position(|c| c == '\n').unwrap() as i16 + 1;
    let p: i16 = input.chars().position(|c| c == '@').expect("Robot🤖 not found!") as i16;
    let pos: Coords = (2*(p % line_len), p / line_len);
    let walls: HashSet<Coords> = input.chars().enumerate().filter(|&(_,c)| c == '#').map(|(i,_)| (2*(i as i16 % line_len), i as i16 / line_len)).collect();
    let boxes: HashSet<Coords> = input.chars().enumerate().filter(|&(_,c)| c == 'O').map(|(i,_)| (2*(i as i16 % line_len), i as i16 / line_len)).collect();
    let moves: Vec<char> = input.split("\n\n").nth(1).unwrap().chars().collect();
    return (pos, walls, boxes, moves);
}

fn step(pos: &mut Coords, walls: &HashSet<Coords>, boxes: &mut HashSet<Coords>, x: i16, y: i16) {
    let mut dsts = vec!((pos.0 + x, pos.1 + y));
    if x == 0 {
        dsts.push((pos.0 - 1, pos.1 + y));
        // dsts.push((pos.0 + 1, pos.1 + y));
    } else if x == -1 {
        dsts = vec!((pos.0 - 2, pos.1 + y));
    }
    for dst in dsts.iter() {
        if boxes.contains(&dst) {
            if !can_push(boxes, walls, &dst, x, y) {
                return
            }
        }
        if walls.contains(&dst) {
            return;
        }
        if boxes.contains(&dst) {
            push(boxes, walls, &dst, x, y);
        }
    }
    pos.0 += x;
    pos.1 += y;
}

fn can_push(boxes: &mut HashSet<Coords>, walls: &HashSet<Coords>, src: &Coords, x: i16, y: i16) -> bool {
    let mut dsts = vec!((src.0 + x, src.1 + y));
    if x == 0 {
        dsts.push((src.0 - 1, src.1 + y));
        dsts.push((src.0 + 1, src.1 + y));
    } else {
        dsts = vec!((src.0 + (2 * x), src.1 + y));
    }
    for dst in dsts.iter() {
        if walls.contains(&dst) {
            // println!("at {:?} wall !!!", dst);
            return false;
        } else {
            // println!("at {:?} no wall", dst);
        }
        if boxes.contains(&dst) {
            if !can_push(boxes, walls, &dst, x, y) {
                return false;
            }
        }
    }
    return true;
}

fn push(boxes: &mut HashSet<Coords>, walls: &HashSet<Coords>, src: &Coords, x: i16, y: i16) -> bool {
    let mut dsts = vec!((src.0 + x, src.1 + y));
    if x == 0 {
        dsts.push((src.0 - 1, src.1 + y));
        dsts.push((src.0 + 1, src.1 + y));
    } else {
        dsts = vec!((src.0 + (2 * x), src.1 + y));
    }
    for dst in dsts.iter() {
        if boxes.contains(&dst) {
            push(boxes, walls, &dst, x, y);
        }
    }
    // println!("moving box {:?} to {:?}", src, (src.0 + x, src.1 + y));
    boxes.remove(src);
    boxes.insert((src.0 + x, src.1 + y));
    return true;
}

fn main() {
  let (mut pos, walls, mut boxes, moves) = parse_input(INPUT);
//   println!("{:?}", pos);
//   println!("{:?}", walls);
//   println!("{:?}", boxes);
  //println!("{:?}", moves);
  
  for d in moves {
    //   println!("------------  {}", d);
      match d {
          '<' => step(&mut pos, &walls, &mut boxes, -1, 0),
          '^' => step(&mut pos, &walls, &mut boxes, 0, -1),
          '>' => step(&mut pos, &walls, &mut boxes, 1, 0),
          'v' => step(&mut pos, &walls, &mut boxes, 0, 1),
          _ => continue
      }
    //   println!("pos: {:?}", pos);
    //   println!("{:?}", walls);
    //   println!("boxes: {:?}", boxes);
   
  }
  let sum: usize = boxes.iter().map(|b| 100 * b.1 as usize + b.0 as usize).sum();
  println!("{}", sum);
   
}