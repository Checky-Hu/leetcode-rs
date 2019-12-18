use std::env;

struct Solution {
}

impl Solution {
    fn get_gcd(a: i32, b: i32) -> i32 {
        let (mut m, mut n) = if a > b {
            (a, b)
        } else {
            (b, a)
        };
        loop {
            let t: i32 = m % n;
            if t == 0 {
                break;
            } else {
                m = n;
                n = t;
            }
        }
        n
    }

    fn add_two_fraction(pre_num: &mut i32, pre_den: &mut i32, num: i32, den: i32, positive: bool) {
        if *pre_den == 0 {
            *pre_num = if positive {
                num
            } else {
                0 - num
            };
            *pre_den = den;
        } else {
            let cur_num = if positive {
                num
            } else {
                0 - num
            };
            *pre_num = *pre_num * den + cur_num * *pre_den;
            *pre_den = *pre_den * den;
            if *pre_num == 0 {
                *pre_den = 1;
                return
            }
            let is_add: bool = if *pre_num < 0 {
                *pre_num = 0 - *pre_num;
                false
            } else {
                true
            };
            let t: i32 = Solution::get_gcd(*pre_num, *pre_den);
            *pre_num = if is_add {
                *pre_num / t
            } else {
                0 - *pre_num / t
            };
            *pre_den /= t;
        }
    }

    pub fn fraction_addition(expression: String) -> String {
        let mut pre_num: i32 = 0;
        let mut pre_den: i32 = 0;
        let mut cur_num: i32 = 0;
        let mut cur_den: i32 = 0;
        let mut is_add: bool = true;
        let mut reach_fs: bool = false;
        for c in expression.chars() {
            match c {
                '+' | '-' => {
                    Solution::add_two_fraction(&mut pre_num, &mut pre_den, cur_num, cur_den, is_add);
                    cur_num = 0;
                    cur_den = 0;
                    if c == '+' {
                        is_add = true;
                    } else {
                        is_add = false;
                    }
                    reach_fs = false;
                },
                '/' => {
                    reach_fs = true;
                },
                _ => {
                    if reach_fs {
                        cur_den = cur_den * 10 + (c as u8 - 48) as i32;
                    } else {
                        cur_num = cur_num * 10 + (c as u8 - 48) as i32;
                    }
                },
            }
        }
        Solution::add_two_fraction(&mut pre_num, &mut pre_den, cur_num, cur_den, is_add);
        pre_num.to_string() + "/" + &pre_den.to_string()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let expression: String = arg;
            println!("Result: {}", Solution::fraction_addition(expression));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
