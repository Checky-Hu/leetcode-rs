use std::env;

struct Solution {
}

impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let row: usize = a.len();
        if row <= 1 {
            return 0
        }

        let column: usize = a[0].len();
        let mut i: usize = 0;
        let mut flags: Vec<&[u8]> = Vec::new();
        while i < row {
            flags.push(a[i].as_bytes());
            i += 1;
        }

        i = 0;
        let mut result: i32 = 0;
        while i < column {
            let mut pre_c: u8 = flags[0][i];
            let mut j: usize = 1;
            while j < row {
                let tmp_c: u8 = flags[j][i];
                if pre_c > tmp_c {
                    result += 1;
                    break;
                } else {
                    pre_c = tmp_c;
                    j += 1;
                }
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            a.push(s);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Deletion: {}", Solution::min_deletion_size(a));
}

