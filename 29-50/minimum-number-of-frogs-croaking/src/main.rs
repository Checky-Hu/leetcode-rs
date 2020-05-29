use std::env;

struct Solution {}

impl Solution {
    fn get_index(c: char) -> usize {
        match c {
            'r' => 1,
            'o' => 2,
            'a' => 3,
            'k' => 4,
            _ => 0,
        }
    }

    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut result: i32 = 0;
        let mut count: Vec<i32> = vec![0; 5];
        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    count[0] += 1;
                    if count[0] > result {
                        result = count[0];
                    }
                }
                _ => {
                    let index: usize = Solution::get_index(c);
                    count[index] += 1;
                    if count[index] > count[index - 1] {
                        return -1;
                    }
                    if c == 'k' {
                        for v in count.iter_mut() {
                            *v -= 1;
                        }
                    }
                }
            }
        }
        if count[0] == 0 {
            result
        } else {
            -1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let croak_of_frogs: String = arg;
            println!(
                "Minimum number of frogs: {}",
                Solution::min_number_of_frogs(croak_of_frogs)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
