use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let mut same: bool = false;
        while fast < len {
            let needed: bool = if slow == 0 {
                true
            } else if nums[fast] == nums[slow - 1] {
                if same {
                    false
                } else {
                    same = true;
                    true
                }
            } else {
                same = false;
                true
            };
            if needed {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(num);
        }
    }

    let len: i32 = Solution::remove_duplicates(&mut nums);
    println!("Size: {}", len);
    for num in nums.iter().take(len as usize) {
        print!("{} ", *num);
    }
    println!();
}
