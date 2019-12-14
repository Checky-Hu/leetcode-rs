use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        let mut tmp: i32 = n;
        while tmp > 0 {
            nums.push(tmp % 10);
            tmp /= 10;
        }

        let len: usize = nums.len();
        let mut index: usize = 0;
        while index < len - 1 {
            if nums[index] > nums[index + 1] {
                let mut tmp_i: usize = 0;
                while tmp_i <= index {
                    if nums[tmp_i] > nums[index + 1] {
                        break;
                    }
                    tmp_i += 1;
                }

                let tmp: i32 = nums[index + 1];
                nums[index + 1] = nums[tmp_i];
                nums[tmp_i] = tmp;
                break;
            }
            index += 1;
        }

        if index == len - 1 {
            -1
        } else {
            let mut offset: usize = 0;
            while offset < (index + 1) / 2 {
                let tmp: i32 = nums[offset];
                nums[offset] = nums[index - offset];
                nums[index - offset] = tmp;
                offset += 1;
            }
            let mut multi: i64 = 1;
            let mut result: i64 = 0;
            for num in nums {
                result += num as i64 * multi;
                multi *= 10;
            }
            if result > i32::max_value() as i64 {
                -1
            } else {
                result as i32
            }
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Next greater number: {}", Solution::next_greater_element(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
