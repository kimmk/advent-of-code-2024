fn str_to_mat(input: &str) -> Vec<Vec<char>> {
    input.lines()
         .map(|line| line.chars().collect::<Vec<char>>())
         .collect()
}

fn cmp_mats(
    a: &Vec<Vec<char>>,
    b: &Vec<Vec<char>>,
) -> bool {
    let w: usize = a[0].len();
    let h: usize = b.len();
    for row_i in 0..w {
        for col_i in 0..h {
            let a_val = a[col_i][row_i];
            let b_val = b[col_i][row_i];
            if b_val == '.' {
                continue;
            }
            if a_val != b_val {
                return false;
            }
        }
    }
    return true;
}

fn find_submat_in_mat(
    mat: &Vec<Vec<char>>,
    smat: &Vec<Vec<char>>,
) -> i32 {
    let mut count = 0;
    let mat_width: usize = mat[0].len();
    let mat_height: usize = mat.len();
    let smat_width: usize = smat[0].len();
    let smat_height: usize = smat.len();
    for row_i in 0..mat_height-smat_height+1 {
        for col_i in 0..mat_width-smat_width+1 {
            let cmp_mat = mat.iter()
                     .skip(row_i)
                     .take(smat_height)
                     .map(|col| col.iter()
                                  .skip(col_i)
                                  .take(smat_width)
                                  .cloned()
                                  .collect::<Vec<char>>())
                     .collect::<Vec<Vec<char>>>();
            println!("x {} y {} {:?}", col_i, row_i, cmp_mat);
            if cmp_mats(&cmp_mat, &smat) {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let mut count = 0;
    let input = include_str!("input");
    let mat = str_to_mat(input);
    let amats = [
        // M.S
        // .A.
        // M.S
        str_to_mat("M.S\n.A.\nM.S"),

        // M.M
        // .A.
        // S.S
        str_to_mat("M.M\n.A.\nS.S"),

        // S.M
        // .A.
        // S.M
        str_to_mat("S.M\n.A.\nS.M"),

        // S.S
        // .A.
        // M.M
        str_to_mat("S.S\n.A.\nM.M"),
    ];
    for amat in amats {
        println!("{:?}", amat);
        count += find_submat_in_mat(&mat, &amat);
    }
    println!("count: {}", count);
}
