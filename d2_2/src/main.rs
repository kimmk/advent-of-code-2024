fn analyze_safety(
    line: &str,
    skip_idx: i32,
) -> i32 {
    let mut idx = 0;
    let mut line_nums = line.split_whitespace().map(str::parse::<i32>);
    let mut dir = Option::<bool>::None;
    let mut prev_num = line_nums.next().unwrap().unwrap();
    if skip_idx == 0 {
        print!("(skipped {}) ", prev_num);
        prev_num = line_nums.next().unwrap().unwrap();
    }
    print!("{} ", prev_num);
    for num in line_nums {
        let cur_num = num.unwrap();
        if idx == skip_idx {
            print!("(skipped {}) ", cur_num);
            idx += 1;
            continue;
        }
        print!("{} ", cur_num);
        let diff = prev_num - cur_num;
        if diff == 0 || diff.abs() > 3 {
            println!("unsafe diff");
            return idx-1;
        }
        let up = diff > 0;
        match (dir, up) {
            (None, d) => { dir = Some(d); }
            (Some(true), true) => {}
            (Some(false), false) => {}
            _ => {
                println!("unsafe dir");
                return idx-1;
            }
        }
        prev_num = cur_num;
        idx += 1;
    }
    println!("OK!");
    return -1;
}

fn main() {
    let input = include_str!("input");
    let lines: Vec<&str> = input.lines().collect();

    let mut safe_count = 0;
    let mut line_num = 0;
    for line in lines {
        println!("line_num {}:\n{}", line_num, line);
        let fail_idx = analyze_safety(line, -1);
        if fail_idx == -1 {
            println!("line_num {} safe by default", line_num);
            safe_count += 1;
        }
        else if analyze_safety(line, fail_idx) == -1 {
            println!("line_num {} safe after removing {}", line_num, fail_idx);
            safe_count += 1;
        }
        else {
            println!("line_num {} not safe:\n{}", line_num, line);
        }
        println!();
        line_num += 1;
    }
    println!("\nsafe: {}", safe_count);
}
