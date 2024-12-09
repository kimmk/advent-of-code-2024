use regex::Regex;

fn main() {
    let input = include_str!("input");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total: i32 = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|cap| cap.extract()) {
        let num_a = a.parse::<i32>().unwrap();
        let num_b = b.parse::<i32>().unwrap();
        total += num_a * num_b;
    }
    println!("total = {}", total);
}
