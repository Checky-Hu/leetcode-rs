use std::env;

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        Vec::new()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            let result: Vec<String> = Solution::find_repeated_dna_sequences(s);
            println!("Repeated dns sequences:");
            for r in result.iter() {
                println!("{}", *r);
            }
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
