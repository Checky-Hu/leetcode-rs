use std::env;

struct Solution {}

impl Solution {
    fn quick_sort(nums: &mut Vec<u8>, left: usize, right: usize, weights: &[i32]) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: u8 = nums[left];
        while i < j {
            while i < j
                && (weights[nums[j] as usize - 97] < 0
                    || weights[nums[j] as usize - 97] >= weights[temp as usize - 97])
            {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j
                && (weights[nums[i] as usize - 97] < 0
                    || weights[nums[i] as usize - 97] <= weights[temp as usize - 97])
            {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = temp;
        if left + 1 < i {
            Solution::quick_sort(nums, left, i - 1, weights);
        }
        Solution::quick_sort(nums, i + 1, right, weights);
    }

    pub fn custom_sort_string(s: String, t: String) -> String {
        let mut weights: Vec<i32> = vec![-1; 26];
        for (i, u) in s.as_bytes().iter().enumerate() {
            weights[*u as usize - 97] = i as i32;
        }
        let mut result: Vec<u8> = t.into_bytes();
        let len: usize = result.len();
        Solution::quick_sort(&mut result, 0, len - 1, &weights);
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: String = arg;
                println!("Custom sort string: {}", Solution::custom_sort_string(s, t));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
