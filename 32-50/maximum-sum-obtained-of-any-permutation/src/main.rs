use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut mut_nums: Vec<i32> = nums;
        mut_nums.sort();
        let len: usize = mut_nums.len();
        let mut counts: Vec<i32> = vec![0; len];
        for r in requests.iter() {
            counts[r[0] as usize] += 1;
            if r[1] as usize + 1 < len {
                counts[r[1] as usize + 1] -= 1;
            }
        }
        for i in 1..len {
            counts[i] += counts[i - 1];
        }
        counts.sort();
        let modulo: i32 = 1_000_000_007;
        let mut result: i32 = 0;
        let mut index: usize = len - 1;
        loop {
            if counts[index] == 0 {
                break;
            } else {
                result = (result + counts[index] * mut_nums[index] % modulo) % modulo;
                if index == 0 {
                    break;
                } else {
                    index -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut requests: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if nums.len() == n {
                    tmp_row.push(t);
                    if tmp_row.len() == 2 {
                        requests.push(tmp_row);
                        tmp_row = Vec::new();
                    }
                } else {
                    nums.push(t);
                }
            }
        }
    }

    if 0 == ret || n == 0 || nums.len() != n || !tmp_row.is_empty() {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    println!(
        "Maximum sum obtained of any permutation: {}",
        Solution::max_sum_range_query(nums, requests)
    );
}
