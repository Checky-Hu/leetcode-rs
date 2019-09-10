use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_i: usize = m as usize;
        let mut nums2_i: usize = n as usize;
        let mut cur_i: usize = nums1_i + nums2_i;
        while nums1_i > 0 {
            if nums2_i > 0 {
                if nums1[nums1_i - 1] > nums2[nums2_i - 1] {
                    nums1[cur_i - 1] = nums1[nums1_i - 1];
                    nums1_i -= 1;
                } else {
                    nums1[cur_i - 1] = nums2[nums2_i - 1];
                    nums2_i -= 1;
                }
            } else {
                nums1[cur_i - 1] = nums1[nums1_i - 1];
                nums1_i -= 1;
            }
            cur_i -= 1;
        }
        while nums2_i > 0 {
            nums1[cur_i - 1] = nums2[nums2_i - 1];
            nums2_i -= 1;
            cur_i -= 1;
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut total: i32 = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut m: i32 = 0;
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => total = i32::from_str(&arg).expect("Error parse."),
            2 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let tmp: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret <= m {
                    nums1.push(tmp);
                } else {
                    nums1.push(0);
                    nums2.push(tmp);
                }
            },
        }
    }

    if 0 == total || 0 == m || total <= m || total != ret {
        println!("Require at least (total + 2) parameters.");
        return;
    }

    Solution::merge(&mut nums1, m, &mut nums2, total - m);
    for t in nums1 {
        print!("{} ", t);
    }
    print!("\n");
}
