fn analyze_safety(
    line: &str,
) -> bool {
    let mut safe = true;
    let mut line_nums = line.split_whitespace().map(str::parse::<i32>);
    let mut prev_num = line_nums.next().unwrap().unwrap();
    let mut dir = Option::<bool>::None;
    for num in line_nums {
        let cur_num = num.unwrap();
        let up = prev_num < cur_num;
        match (dir, up) {
            (Some(true), true) => {}
            (Some(false), false) => {}
            (None, d) => { dir = Some(d); }
            _ => { safe = false; }
        }
        let diff = (prev_num - cur_num).abs();
        if diff == 0 || diff > 3 {
            safe = false;
        }
        if !safe {
            break;
        }
        prev_num = cur_num;
    }
    return safe;
}

fn main() {
    let input = include_str!("input");
    let lines: Vec<&str> = input.lines().collect();

    let mut safe_count = 0;
    for line in lines {
        if analyze_safety(line) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
}
