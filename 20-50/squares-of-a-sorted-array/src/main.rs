use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let len: usize = a.len();
        let mut result: Vec<i32> = Vec::new();
        let mut pos_i: usize = 0;
        while pos_i < len {
            if a[pos_i] >= 0 {
                break;
            } else {
                pos_i += 1;
            }
        }
        if pos_i == 0 {
            for n in a {
                result.push(n * n);
            }
        } else if pos_i == len {
            pos_i = len - 1;
            loop {
                result.push(a[pos_i] * a[pos_i]);
                if pos_i == 0 {
                    break;
                } else {
                    pos_i -= 1;
                }
            }
        } else {
            let mut i: usize = pos_i;
            let mut j: usize = pos_i - 1;
            while i < len {
                if j < pos_i {
                    let tmp: i32 = a[i] + a[j];
                    if tmp == 0 {
                        result.push(a[i] * a[i]);
                        i += 1;
                        result.push(a[j] * a[j]);
                        if j == 0 {
                            j = pos_i;
                        } else {
                            j -= 1;
                        }
                    } else if tmp < 0 {
                        result.push(a[i] * a[i]);
                        i += 1;
                    } else {
                        result.push(a[j] * a[j]);
                        if j == 0 {
                            j = pos_i;
                        } else {
                            j -= 1;
                        }
                    }
                } else {
                    result.push(a[i] * a[i]);
                    i += 1;
                }
            }
            if j != pos_i {
                loop {
                    result.push(a[j] * a[j]);
                    if j == 0 {
                        break;
                    } else {
                        j -= 1;
                    }
                }
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
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::sorted_squares(a);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}

