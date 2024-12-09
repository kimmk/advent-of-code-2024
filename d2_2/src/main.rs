fn analyze_safety(
    line: &Vec<i32>,
    skip_idx: i32,
) -> bool {
    let mut idx = 1;
    let mut last_dir = Option::<bool>::None;
    let mut line_nums = line.iter();
    let mut prev_num = line_nums.next().unwrap();
    if skip_idx == 0 {
        print!("(skipped {}) ", prev_num);
        prev_num = line_nums.next().unwrap();
    }
    print!("{} ", prev_num);
    for cur_num in line_nums {
        if idx == skip_idx {
            print!("(skipped {}) ", cur_num);
            idx += 1;
            continue;
        }
        print!("{} ", cur_num);
        let diff = prev_num - cur_num;
        if diff == 0 || diff.abs() > 3 {
            println!("unsafe diff");
            return false;
        }
        let dir = diff > 0;
        match (last_dir, dir) {
            (None, d) => { last_dir = Some(d); }
            (Some(true), true) => {}
            (Some(false), false) => {}
            _ => {
                println!("unsafe dir");
                return false;
            }
        }
        prev_num = cur_num;
        idx += 1;
    }
    println!("OK!");
    return true;
}

fn main() {
    let input = include_str!("input");
    let lines: Vec<&str> = input.lines().collect();
    let mut safe_count = 0;
    let mut line_num = 0;
    for line in lines {
        println!("line_num {}:\n{}", line_num, line);
        let line_nums: Vec<i32> = line.split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect();
        for idx in -1..line_nums.len() as i32 {
            if analyze_safety(&line_nums, idx) {
                safe_count += 1;
                println!("line_num {} is safe after removing index {}", line_num, idx);
                break;
            }
        }
        println!("\n");
        line_num += 1;
    }
    println!("\nsafe: {}", safe_count);
}
