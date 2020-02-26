use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        // 0 => [4, 6], 1 => [6, 8], 2 => [7, 9], 3 => [4, 8]
        // 4 => [0, 3, 9], 6 => [0, 1, 7]
        // 7 => [2, 6], 8 => [1, 3], 9 => [2, 4]
        let modulo: i32 = 1_000_000_007;
        let mut pre: Vec<i32> = vec![1; 10];
        for _i in 0..(n - 1) {
            let mut tmp: Vec<i32> = vec![0; 10];
            tmp[0] = (pre[4] + pre[6]) % modulo;
            tmp[1] = (pre[6] + pre[8]) % modulo;
            tmp[2] = (pre[7] + pre[9]) % modulo;
            tmp[3] = (pre[4] + pre[8]) % modulo;
            tmp[4] = ((pre[0] + pre[3]) % modulo + pre[9]) % modulo;
            tmp[5] = 0;
            tmp[6] = ((pre[0] + pre[1]) % modulo + pre[7]) % modulo;
            tmp[7] = (pre[2] + pre[6]) % modulo;
            tmp[8] = (pre[1] + pre[3]) % modulo;
            tmp[9] = (pre[2] + pre[4]) % modulo;
            pre = tmp;
        }
        let mut result: i32 = 0;
        for v in pre.iter() {
            result = (result + *v) % modulo;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Count of dialers: {}", Solution::knight_dialer(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
