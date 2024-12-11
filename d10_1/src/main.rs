
struct TopoMap {
    map: Vec<Vec<u32>>,
    w: u32,
    h: u32,
}

impl TopoMap {
    fn h(&self, x: u32, y: u32) -> u32 {
        return self.map[y as usize][x as usize];
    }
}

fn topo_build(
    topo_str: &str
) -> TopoMap {
    let mut map = Vec::new();
    for topo_line in topo_str.lines() {
        let mut row = Vec::new();
        for h in topo_line.chars().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<_>>() {
            print!("{}", h);
            row.push(h);
        }
        map.push(row);
        println!();
    }
    TopoMap {
        map: map.clone(),
        w: map[0].len() as u32,
        h: map.len() as u32,
    }
}

fn walk_trail(
    topo: &TopoMap,
    visited: &mut Vec<Vec<bool>>,
    pos: (u32,u32),
    r: i32
) -> u32 {
    let mut peaks = 0;
    let h = topo.h(pos.0, pos.1);
    let pad = " ".repeat(r as usize);
    if visited[pos.1 as usize][pos.0 as usize] {
        return 0;
    }
    visited[pos.1 as usize][pos.0 as usize] = true;
    if h == 9 {
        peaks = 1;
        //println!("{}found top at {}.{} {}", pad, pos.0, pos.1, h);
    }
    else {
        //println!("{}walking trail {}.{} {}", pad, pos.0, pos.1, h);
        if pos.1 > 0 && topo.h(pos.0, pos.1-1) == h + 1 {
            //println!("{}walking north", pad);
            peaks += walk_trail(&topo, visited, (pos.0, pos.1-1), r+1);
        }
        if pos.0 < topo.w-1 && topo.h(pos.0+1, pos.1) == h + 1 {
            //println!("{}walking east", pad);
            peaks += walk_trail(&topo, visited, (pos.0+1, pos.1), r+1);
        }
        if pos.1 < topo.h-1 && topo.h(pos.0, pos.1+1) == h + 1 {
            //println!("{}walking south", pad);
            peaks += walk_trail(&topo, visited, (pos.0, pos.1+1), r+1);
        }
        if pos.0 > 0 && topo.h(pos.0-1, pos.1) == h + 1 {
            //println!("{}walking west", pad);
            peaks += walk_trail(&topo, visited, (pos.0-1, pos.1), r+1);

        }
    }
    if h == 0 {
        //println!("found {} trails at {}.{}", peaks, pos.0, pos.1);
    }
    return peaks;
}

fn find_trails(
    topo: &mut TopoMap,
) -> u32 {
    let mut trails = 0;
    for (y, row) in topo.map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if *h == 0 {
                //println!("\n\nstarting trail at {}.{}", x, y);
                let mut visited = vec![vec![false; topo.w as usize]; topo.h as usize];
                trails += walk_trail(topo, &mut visited, (x as u32, y as u32), 0);
            }
        }
    }
    return trails;
}

fn main() {
    let input = include_str!("input");
    let mut topo = topo_build(&input);
    println!();
    let trails = find_trails(&mut topo);
    println!("trails: {}", trails);
}
