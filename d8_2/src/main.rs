use std::collections::HashMap;
use std::collections::HashSet;
use num::integer::gcd;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Antenna {
    ch: char,
    pos: (i32,i32),
}

struct Map {
    ants: HashMap<char, Vec<Antenna>>,
    chars: Vec<Vec<char>>,
    w: i32,
    h: i32,
}

fn generate_map(
    lines: &Vec<&str>,
) -> Map {
    let mut map = Map {
        ants: HashMap::new(),
        chars: vec![],
        w: lines[0].len() as i32,
        h: lines.len() as i32,
    };
    for (y, row) in lines.iter().enumerate() {
        let map_row = row.chars().collect();
        map.chars.push(map_row);
        for (x, c) in row.chars().into_iter().enumerate() {
            if c != '.' {
                map.ants.entry(c).or_insert(Vec::new()).push(Antenna {
                    ch: c,
                    pos: (x as i32, y as i32),
                });
            }
        }
    }
    return map;
}

fn pos_diff(
    pos_a: (i32,i32),
    pos_b: (i32,i32),
) -> (i32,i32) {
    (pos_a.0 - pos_b.0, pos_a.1 - pos_b.1)
}

fn pos_add(
    pos_a: (i32,i32),
    pos_b: (i32,i32),
    limits: (i32,i32),
) -> Option<(i32,i32)> {
    let new_pos = (pos_a.0 + pos_b.0, pos_a.1 + pos_b.1);
    if  new_pos.0 < 0 ||
        new_pos.0 >= limits.0 ||
        new_pos.1 < 0 ||
        new_pos.1 >= limits.1 {
        return None
    }
    Some(new_pos)
}

fn pos_calc_antinodes(
    map: &Map,
    pos: (i32,i32)
) -> Vec<(i32,i32)> {
    let mut antinodes = vec![];
    let ant_char = map.chars[pos.1 as usize][pos.0 as usize];
    if ant_char != '.' {
        let ants = map.ants.get(&ant_char).unwrap();
        for ant in ants {
            let pos_d = pos_diff(pos, ant.pos);
            if pos_d.0 == 0 && pos_d.1 == 0 {
                continue;
            }
            let gcd_d = gcd(pos_d.0, pos_d.1);
            let step_d = (pos_d.0 / gcd_d, pos_d.1 / gcd_d);
            let mut cur_pos = ant.pos;
            loop {
                let new_pos = pos_add(cur_pos, step_d, (map.w, map.h));
                if let Some(new_ant_pos) = new_pos {
                    antinodes.push(new_ant_pos);
                    cur_pos = new_ant_pos;
                }
                else {
                    break;
                }
            }

        }
    }
    antinodes
}

fn map_calc_antinodes(
    map: &Map,
) -> HashSet<(i32,i32)> {
    let mut antinodes = HashSet::new();
    for y in 0..map.h {
        for x in 0..map.w {
            let pos_antinodes = pos_calc_antinodes(&map, (x,y));
            for pos_an in pos_antinodes {
                antinodes.insert(pos_an);
            }
        }
    }
    return antinodes;
}

fn print_map(
    map: &Map,
    antinodes: &HashSet<(i32,i32)>
) {
    for y in 0..map.h {
        for x in 0..map.w {
            let mut was_antinode = false;
            for antinode in antinodes {
                if *antinode == (x, y) {
                    print!("#");
                    was_antinode = true;
                    break;
                }
            }
            if !was_antinode {
                print!("{}", map.chars[y as usize][x as usize]);
            }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", input);
    let map_lines = input.lines().collect();
    let map = generate_map(&map_lines);
    let antinodes = map_calc_antinodes(&map);
    print_map(&map, &antinodes);
    println!("antinode count: {}", antinodes.len());
}
