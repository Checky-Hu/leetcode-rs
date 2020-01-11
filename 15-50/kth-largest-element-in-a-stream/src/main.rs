extern crate quicksort;

use quicksort::qsi32;

struct KthLargest {
    k_: usize,
    v_: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let len: usize = nums.len();
        let mut tmp: Vec<i32> = nums;
        if len > 1 {
            qsi32::quick_sort_descend(&mut tmp, 0, len - 1);
        }
        KthLargest {
            k_: k as usize,
            v_: tmp,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        let mut add: bool = false;
        for (i, v) in self.v_.iter().enumerate() {
            if *v < val {
                self.v_.insert(i, val);
                add = true;
                break;
            }
        }
        if !add {
            self.v_.push(val);
        }
        self.v_[self.k_ - 1]
    }
}

fn main() {
    let mut obj: KthLargest = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("result of add 3: {}", obj.add(3));
    println!("result of add 5: {}", obj.add(5));
    println!("result of add 10: {}", obj.add(10));
    println!("result of add 9: {}", obj.add(9));
    println!("result of add 4: {}", obj.add(4));
}
