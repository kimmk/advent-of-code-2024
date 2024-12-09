fn str_to_mat(input: &str) -> Vec<Vec<char>> {
    input.lines()
         .map(|line| line.chars().collect::<Vec<char>>())
         .collect()
}

fn find_str_in_row(
    row: &Vec<char>,
    target: &str,
) -> i32 {
    let target_reverse = target.chars().rev().collect::<String>();
    let mut count = 0;
    if row.len() < target.len() {
        return 0;
    }

    println!("row: {:?}", row);
    for i in 0..=(row.len()-target.len()) {
        let substr: String = row[i..i+target.len()].iter().collect();
        if substr == target {
            count += 1;
        }
        else if substr == target_reverse {
            count += 1;
        }
        //println!("searched substr: {}", substr);
    }
    //println!("");
    return count;
}

fn get_diag(
    mat: &Vec<Vec<char>>,
    row_start: usize,
    clm_start: usize,
    down: bool,
    right: bool,
) -> Vec<char> {
    let width: usize = mat[0].len();
    let height: usize = mat.len();
    let mut diag = Vec::new();
    let mut clm_i = clm_start;
    let mut row_i = row_start;
    loop {
        diag.push(mat[row_i][clm_i]);
        if right {
            if clm_i == width-1 {
                break;
            }
            clm_i += 1;
        }
        else {
            if clm_i == 0 {
                break;
            }
            clm_i -= 1;
        }
        if down {
            if row_i == height-1 {
                break;
            }
            row_i += 1;
        }
        else {
            if row_i == 0 {
                break;
            }
            row_i -= 1;
        }
    }
    return diag;
}

fn find_str(
    mat: Vec<Vec<char>>,
    target: &str,
) -> i32 {
    let mut count = 0;
    let width = mat[0].len();
    let height = mat.len();
    println!("ROWS");
    for row in &mat {
        count += find_str_in_row(row, target);
    }
    println!("\nCOLUMNS");
    for col in 0..width {
        let col_str = mat.iter().map(|row| row[col]).collect();
        count += find_str_in_row(&col_str, target);
    }
    println!("\nLEFT ROW DIAGONALS");
    for row_i in 0..height {
        println!("row_i: {}", row_i);
        let diag_up = get_diag(&mat, row_i, 0, false, true);
        count += find_str_in_row(&diag_up, target);
        let diag_dn = get_diag(&mat, row_i, 0, true, true);
        count += find_str_in_row(&diag_dn, target);
    }
    println!("\n TOP AND BOT COLUMN RIGHT DIAGONALS");
    for col_i in 1..width {
        println!("col_i: {}", col_i);
        let diag_top = get_diag(&mat, 0, col_i, true, true);
        count += find_str_in_row(&diag_top, target);
        let diag_bot = get_diag(&mat, height-1, col_i, false, true);
        count += find_str_in_row(&diag_bot, target);
    }
    return count;
}

fn main() {
    let input = include_str!("input");
    let mat = str_to_mat(input);
    let count = find_str(mat, "XMAS");
    println!("count: {}", count);
}
