use std::collections::HashMap;

struct FindSumPairs {
    nums2_: Vec<i32>,
    maps1_: HashMap<i32, i32>,
    maps2_: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut maps1: HashMap<i32, i32> = HashMap::new();
        for num in nums1.iter() {
            match maps1.get_mut(num) {
                Some(x) => *x += 1,
                None => {
                    maps1.insert(*num, 1);
                }
            }
        }
        let mut maps2: HashMap<i32, i32> = HashMap::new();
        for num in nums2.iter() {
            match maps2.get_mut(num) {
                Some(x) => *x += 1,
                None => {
                    maps2.insert(*num, 1);
                }
            }
        }
        FindSumPairs {
            nums2_: nums2,
            maps1_: maps1,
            maps2_: maps2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let t: i32 = self.nums2_[index as usize] + val;
        if let Some(x) = self.maps2_.get_mut(&self.nums2_[index as usize]) {
            *x -= 1;
        }
        match self.maps2_.get_mut(&t) {
            Some(x) => *x += 1,
            None => {
                self.maps2_.insert(t, 1);
            }
        }
        self.nums2_[index as usize] = t;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut result: i32 = 0;
        for (k, v) in self.maps1_.iter() {
            if let Some(x) = self.maps2_.get(&(tot - k)) {
                result += *x * v;
            }
        }
        result
    }
}

fn main() {
    let mut obj: FindSumPairs = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
    println!("Count 7: {}", obj.count(7));
    obj.add(3, 2);
    println!("Count 8: {}", obj.count(8));
    println!("Count 4: {}", obj.count(4));
    obj.add(0, 1);
    obj.add(1, 1);
    println!("Count 7: {}", obj.count(7));
}
