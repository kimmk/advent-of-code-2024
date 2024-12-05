use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    let mut tokens = input.split_whitespace();
    let mut nums_left: Vec<i32> = Vec::new();
    let mut nums_right: Vec<i32> = Vec::new();

    loop {
        let token_left = tokens.next();
        if token_left == None {
            break;
        }
        let token_right = tokens.next();
        nums_left.push(token_left.unwrap().parse::<i32>().unwrap());
        nums_right.push(token_right.unwrap().parse::<i32>().unwrap());
    }

    let mut right_counts = HashMap::<i32, i32>::new();

    for num in nums_right {
        let count = right_counts.entry(num).or_insert(0);
        *count += 1;
    }

    let mut score = 0;

    for num in nums_left {
        let count = right_counts.get(&num).unwrap_or(&0);
        score += count * num;
    }

    println!("{}", score);
}
