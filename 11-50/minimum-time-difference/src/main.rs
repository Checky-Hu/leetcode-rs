use std::env;

struct Solution {
}

impl Solution {
    pub fn compare_vec(a: &Vec<u8>, b: &Vec<u8>) -> bool {
        if a[0] > b[0] {
            true
        } else if a[0] < b[0] {
            false
        } else {
            a[1] >= b[1]
        }
    }

    pub fn quick_sort_by_func(nums: &mut Vec<Vec<u8>>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: Vec<u8> = nums[left].clone();
        while i < j {
            while i < j && Solution::compare_vec(&nums[j], &temp) {
                j -= 1;
            }
            nums[i][0] = nums[j][0];
            nums[i][1] = nums[j][1];
            while i < j && !Solution::compare_vec(&nums[i], &temp) {
                i += 1;
            }
            nums[j][0] = nums[i][0];
            nums[j][1] = nums[i][1];
        }
        nums[i][0] = temp[0];
        nums[i][1] = temp[1];
        if left + 1 < i {
            Solution::quick_sort_by_func(nums, left, i - 1);
        }
        Solution::quick_sort_by_func(nums, i + 1, right);
    }

    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let len: usize = time_points.len();
        let mut tmp: Vec<Vec<u8>> = Vec::with_capacity(len);
        for time_point in time_points {
            let bytes: &[u8] = time_point.as_bytes();
            tmp.push(vec![(bytes[0] - 48) * 10 + bytes[1] - 48, (bytes[3] - 48) * 10 + bytes[4] - 48]);
        }
        Solution::quick_sort_by_func(&mut tmp, 0, len - 1);
        let mut result: i32 = i32::max_value();
        for i in 1..len {
            let t: i32 = (tmp[i][0] - tmp[i - 1][0]) as i32 * 60 + tmp[i][1] as i32 - tmp[i - 1][1] as i32;
            if t < result {
                result = t;
            }
        }
        let t: i32 = (24 + tmp[0][0] - tmp[len - 1][0]) as i32 * 60 + tmp[0][1] as i32 - tmp[len - 1][1] as i32;
        if t < result {
            result = t;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut time_points: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                time_points.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Min difference: {}", Solution::find_min_difference(time_points));
}
