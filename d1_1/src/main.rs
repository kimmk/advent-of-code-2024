use std::iter::zip;

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

    nums_left.sort();
    nums_right.sort();

    let num_pairs = zip(&nums_left, &nums_right);
    let mut num_total_diff = 0;

    for (left, right) in num_pairs {
        num_total_diff += (left - right).abs();
    }

    println!("{}", num_total_diff);
}
