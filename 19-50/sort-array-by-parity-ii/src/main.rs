use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let len: usize = a.len();
        let mut result: Vec<i32> = vec![0; len];
        let mut even_pos: usize = 0;
        let mut odd_pos: usize = 1;
        while even_pos < len || odd_pos < len {
            while even_pos < len && a[even_pos] & 1 == 0 {
                result[even_pos] = a[even_pos];
                even_pos += 2;
            }
            while odd_pos < len && a[odd_pos] & 1 == 1 {
                result[odd_pos] = a[odd_pos];
                odd_pos += 2;
            }
            if even_pos < len {
                result[even_pos] = a[odd_pos];
                result[odd_pos] = a[even_pos];
                even_pos += 2;
                odd_pos += 2;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	        a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::sort_array_by_parity_ii(a);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}

