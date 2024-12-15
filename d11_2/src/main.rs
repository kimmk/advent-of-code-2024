use std::collections::HashMap;

fn parse_stones(
    stones_str: &str
) -> Vec<u64> {
    let stones = stones_str.trim_end().split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    return stones;
}

fn blink(
    stones: &Vec<u64>
) -> Vec<u64> {
    let mut new_st = Vec::new();
    for st in stones {
        let st_str = format!("{}", st);
        if *st == 0 {
            new_st.push(1);
        }
        else if (st_str.len() & 1 as usize) == 0 {
            let len = st_str.len();
            let a = &st_str[0..len/2].parse().unwrap();
            new_st.push(*a);
            let b = &st_str[len/2..len].parse().unwrap();
            new_st.push(*b);
        }
        else {
            new_st.push(*st * 2024);
        }
    }
    return new_st;
}

fn build_stone_lib(
    lib: &mut HashMap::<u64,u64>,
    stone: u64,
    depth: u64,
) -> u64 {
    let mut stones = vec![stone];
    for _ in 0..depth {
        stones = blink(&stones);
    }
    lib.insert(stone, stones.len() as u64);
    stones.len() as u64
}

fn main() {
    let input = include_str!("input");
    let mut stones = parse_stones(input);
    println!("initial: {:?}", stones);
    let mut stone_lib = HashMap::<u64,u64>::new();
    let lib_depth = 37;
    for level in 0..=37 {
        println!("calc level {}", level);
        stones = blink(&stones);
    }
    let mut total_stones = 0u64;
    println!("stones to calc using lib: {}", stones.len());
    for (i, stone) in stones.iter().enumerate() {
        if let Some(stone_count) = stone_lib.get(stone) {
            total_stones += stone_count;
            println!("({}/{}) lib used for {}", i, stones.len(), stone);
        }
        else {
            println!("({}/{}) calcing lib value for {}", i, stones.len(), stone);
            total_stones += build_stone_lib(&mut stone_lib, *stone, lib_depth);
        }
    }
    println!("final stone count: {}", total_stones);
}
