
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
            let a = &st_str[0..len/2];
            new_st.push(a.parse().unwrap());
            let b = &st_str[len/2..len];
            new_st.push(b.parse().unwrap());
        }
        else {
            new_st.push(*st * 2024);
        }
    }
    return new_st;
}

fn main() {
    let input = include_str!("input");
    let mut stones = parse_stones(input);
    println!("initial: {:?}", stones);
    for n in 1..=25 {
        stones = blink(&stones);
        //println!("blink {}: {:?}", n, stones);
        //std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    println!("stones: {}", stones.len());
}
