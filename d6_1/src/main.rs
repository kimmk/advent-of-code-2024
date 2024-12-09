
struct Tile {
    is_wall: bool,
    visited: bool,
}

type TileMap = Vec<Vec<Tile>>;

enum GuardDir {
    Up,
    Right,
    Down,
    Left,
}

fn turn_guard(
    dir: GuardDir,
) -> GuardDir {
    match dir {
        GuardDir::Up    => return GuardDir::Right,
        GuardDir::Right => return GuardDir::Down,
        GuardDir::Down  => return GuardDir::Left,
        GuardDir::Left  => return GuardDir::Up,
    }
}

struct GuardMap {
    map: TileMap,
    guard_pos: (i32, i32),
    guard_dir: GuardDir,
    w: i32,
    h: i32,
}

fn next_pos(
    map: &GuardMap
) -> Option<(i32,i32)> {
    let d = match map.guard_dir {
        GuardDir::Up    => (0, -1),
        GuardDir::Right => (1, 0),
        GuardDir::Down  => (0, 1),
        GuardDir::Left  => (-1, 0),
    };
    let new_pos = (map.guard_pos.0 + d.0, map.guard_pos.1 + d.1);
    if (0..map.w).contains(&new_pos.0) && (0..map.h).contains(&new_pos.1) {
        return Some(new_pos);
    }
    None
}

fn generate_map(
    lines: &Vec<&str>,
) -> GuardMap {
    let mut map = GuardMap {
        map: Vec::new(),
        guard_pos: (0, 0),
        guard_dir: GuardDir::Up,
        w: lines[0].len() as i32,
        h: lines.len() as i32,
    };
    for (y, row) in lines.iter().enumerate() {
        let mut map_row = Vec::<Tile>::new();
        for (x, c) in row.chars().into_iter().enumerate() {
            let mut tile = Tile {
                is_wall: c == '#',
                visited: false,
            };
            if c == '^' {
                map.guard_pos = (x as i32, y as i32);
                tile.visited = true;
            }
            map_row.push(tile);
        }
        map.map.push(map_row);
    }
    return map;
}

fn print_map(
    map: &GuardMap,
) {
    for (y, row) in map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if (x as i32, y as i32) == map.guard_pos { print!("^"); }
            else if tile.visited { print!("x"); }
            else if tile.is_wall { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input");
    let map_lines = input.lines().collect();
    let mut map = generate_map(&map_lines);
    let mut visited_count = 1;
    while let Some(new_pos) = next_pos(&map) {
        print_map(&map);
        println!();
        let new_tile = &mut map.map[new_pos.1 as usize][new_pos.0 as usize];
        if new_tile.is_wall {
            map.guard_dir = turn_guard(map.guard_dir);
            continue;
        }
        else if !new_tile.visited {
            new_tile.visited = true;
            visited_count += 1;
        }
        map.guard_pos = new_pos;
    }
    println!("visited tiles: {}", visited_count);
}
