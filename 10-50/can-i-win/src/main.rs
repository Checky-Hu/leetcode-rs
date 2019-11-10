use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    fn can_i_win_loop(len: i32, total: i32, used: i32, map: &mut HashMap<i32, bool>) -> bool {
        match map.get(&used) {
            Some(x) => *x,
            None => {
                for i in 0..len {
                    let cur: i32 = 1 << i;
                    if cur & used == 0 {
                        if total <= i + 1 || !Solution::can_i_win_loop(len, total - i - 1, cur | used, map) {
                            map.insert(used, true);
                            return true
                        }
                    }
                }
                map.insert(used, false);
                false
            },
        }
    }

    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if max_choosable_integer >= desired_total {
            return true
        } else if (max_choosable_integer + 1) * max_choosable_integer / 2 < desired_total {
            return false
        } else {
            let mut map: HashMap<i32, bool> = HashMap::new();
            Solution::can_i_win_loop(max_choosable_integer, desired_total, 0, &mut map)
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut max_choosable_integer: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            max_choosable_integer = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            let desired_total: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Can I win: {}", Solution::can_i_win(max_choosable_integer, desired_total));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }
}
