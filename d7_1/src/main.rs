
fn calc_ops(
    nums: &Vec<i64>,
    result: i64
) -> bool {
    let mut ret = false;
    let sum = nums[0] + nums[1];
    let pro = nums[0] * nums[1];
    if nums.len() == 2 {
        ret |= sum == result;
        ret |= pro == result;
    }
    else {
        let mut sum_nums = vec![sum];
        sum_nums.extend_from_slice(&nums[2..]);
        ret |= calc_ops(&sum_nums, result);
        let mut pro_nums = vec![pro];
        pro_nums.extend_from_slice(&nums[2..]);
        ret |= calc_ops(&pro_nums, result);
    }
    return ret;
}

fn main() {
    let input = include_str!("input_trivial");
    let mut total_calib = 0;
    for input_line in input.lines() {
        let (result_str, operands_str) = input_line.split_once(": ").unwrap();
        let result = result_str.parse::<i64>().unwrap();
        let operands = operands_str.split(" ").map(|op| op.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        println!("operands: {:?}", operands);
        if calc_ops(&operands, result) {
            println!("result found");
            total_calib += result;
        }
    }
    println!("total calib: {}", total_calib);
}
