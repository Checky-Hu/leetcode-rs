use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let len: usize = values.len();
        let mut sort: Vec<(i32, i32)> = Vec::new();
        for i in 0..len {
            sort.push((values[i], labels[i]));
        }
        sort.sort_by(|a, b| b.0.cmp(&a.0));
        let mut result: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut num: i32 = 0;
        for item in sort.iter().take(len) {
            match map.get_mut(&item.1) {
                Some(x) => {
                    if *x < use_limit {
                        *x += 1;
                        result += item.0;
                        num += 1;
                    }
                }
                None => {
                    if 0 < use_limit {
                        map.insert(item.1, 1);
                        result += item.0;
                        num += 1;
                    }
                }
            }
            if num == num_wanted {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut num_wanted: i32 = 0;
    let mut use_limit: i32 = 0;
    let mut n: i32 = 0;
    let mut values: Vec<i32> = Vec::new();
    let mut labels: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => num_wanted = i32::from_str(&arg).expect("Error parse."),
            2 => use_limit = i32::from_str(&arg).expect("Error parse."),
            3 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if values.len() == n as usize {
                    labels.push(number);
                } else {
                    values.push(number);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least (3 + 2 * arg3) parameters.");
        return;
    }

    println!(
        "Largest values from labels: {}",
        Solution::largest_vals_from_labels(values, labels, num_wanted, use_limit)
    );
}
