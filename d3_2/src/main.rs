use regex::Regex;

fn main() {
    let input = include_str!("input_trivial");
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut total: i32 = 0;
    for cap in re.captures_iter(input) {
        let cap_str = cap.get(0).unwrap().as_str();
        if cap_str == "do()" {
            enabled = true;
        }
        else if (cap_str == "don't()") {
            enabled = false;
        }
        else if enabled {
            let a = cap.get(1).unwrap().as_str();
            let b = cap.get(2).unwrap().as_str();
            let num_a = a.parse::<i32>().unwrap();
            let num_b = b.parse::<i32>().unwrap();
            total += num_a * num_b;
        }
    }
    println!("total = {}", total);
}
