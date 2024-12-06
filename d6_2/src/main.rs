use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum GuardDir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq)]
struct Tile {
    is_wall: bool,
    visited: bool,
    visited_dirs: HashSet<GuardDir>,
    x: i32,
    y: i32,
    obstacle: bool,
}

type TileMap = Vec<Vec<Tile>>;

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

impl GuardMap {
    fn ref_tile(
        &mut self,
        pos: (i32, i32)
    ) -> &mut Tile {
        &mut self.map[pos.1 as usize][pos.0 as usize]
    }

    fn ref_tile_const(
        &self,
        pos: (i32, i32)
    ) -> &Tile {
        &self.map[pos.1 as usize][pos.0 as usize]
    }
}

fn next_pos(
    map: &GuardMap,
    dir: GuardDir,
    pos: (i32, i32),
) -> Option<(i32,i32)> {
    let d = match dir {
        GuardDir::Up    => (0, -1),
        GuardDir::Right => (1, 0),
        GuardDir::Down  => (0, 1),
        GuardDir::Left  => (-1, 0),
    };
    let new_pos = (pos.0 + d.0, pos.1 + d.1);
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
                visited_dirs: HashSet::new(),
                x: x as i32,
                y: y as i32,
                obstacle: false,
            };
            if c == '^' {
                map.guard_pos = (x as i32, y as i32);
                tile.visited = true;
                tile.visited_dirs.insert(GuardDir::Up);
            }
            map_row.push(tile);
        }
        map.map.push(map_row);
    }
    return map;
}

fn print_map(
    map: &GuardMap,
    raycast: &Vec<(i32,i32)>,
) {
    for (y, row) in map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.obstacle { print!("Q"); }
            else if (x as i32, y as i32) == map.guard_pos { print!("^"); }
            else if raycast.contains(&(x as i32, y as i32)) { print!("o"); }
            else if tile.is_wall { print!("#"); }
            else if tile.visited { print!("x"); }
            else { print!("."); }
        }
        println!();
    }
}

fn raycast_guard_tiles(
    map: &GuardMap,
    dir: GuardDir,
) -> Vec<(i32,i32)> {
    let mut pos = map.guard_pos;
    let mut tiles = Vec::new();
    loop {
        tiles.push(pos);
        let next_pos = next_pos(map, dir, pos);
        if next_pos.is_none() {
            break;
        }
        pos = next_pos.unwrap();
        if map.ref_tile_const(next_pos.unwrap()).is_wall {
            break;
        }
    }
    return tiles;
}

fn main() {
    let input = include_str!("input");
    let map_lines = input.lines().collect();
    let mut map = generate_map(&map_lines);
    let mut visited_count = 1;
    let mut loop_count = 0;
    let mut place_obstacle = false;
    while let Some(new_pos) = next_pos(&map, map.guard_dir, map.guard_pos) {
        if map.ref_tile(new_pos).is_wall {
            map.guard_dir = turn_guard(map.guard_dir);
            let copy_dir = map.guard_dir.clone();
            map.ref_tile(new_pos).visited_dirs.insert(copy_dir);
            place_obstacle = false;
            continue;
        }
        if place_obstacle {
            loop_count += 1;
            map.ref_tile(new_pos).obstacle = true;
            place_obstacle = false;
        }
        if !map.ref_tile(new_pos).visited {
            map.ref_tile(new_pos).visited = true;
            visited_count += 1;
        }
        map.guard_pos = new_pos;
        let copy_dir = map.guard_dir.clone();
        map.ref_tile(new_pos).visited_dirs.insert(copy_dir);
        let raycast_dir = turn_guard(map.guard_dir);
        let raycast_tiles = raycast_guard_tiles(&map, raycast_dir);
        for rt in &raycast_tiles {
            if map.ref_tile(*rt).visited_dirs.contains(&raycast_dir) {
                place_obstacle = true;
                break;
            }
        }
        print_map(&map, &raycast_tiles);
        println!();
    }
    println!("visited tiles: {}", visited_count);
    println!("loop count: {}", loop_count);
}
