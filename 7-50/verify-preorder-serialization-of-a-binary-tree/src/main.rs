use std::env;

struct Solution {}

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut count_null: i32 = 0;
        let mut count_node: i32 = 0;
        let mut is_node: bool = false;
        for c in preorder.chars() {
            match c {
                ',' => is_node = false,
                '#' => {
                    if count_null == 0 {
                        count_null += 1;
                    } else {
                        count_node -= 1;
                        if count_node < 0 {
                            return false;
                        }
                    }
                }
                _ => {
                    if count_null > count_node {
                        return false;
                    }
                    if !is_node {
                        is_node = true;
                        count_node += 1;
                    }
                }
            }
        }
        count_node == 0
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let preorder: String = arg;
            println!(
                "Valid binary tree: {}",
                Solution::is_valid_serialization(preorder)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
