struct NumArray {
    nums_: Vec<i32>,
    sums_: Vec<i32>,
    size_: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let len: usize = nums.len();
        if len == 0 {
            return NumArray {
                nums_: Vec::new(),
                sums_: Vec::new(),
                size_: 0,
            }
        }
        let l: f64 = len as f64;
        let size: usize = (l / l.sqrt()).ceil() as usize;
        let mut sums: Vec<i32> = vec![0; size];
        for i in 0..len {
            sums[i / size] += nums[i];
        }
        NumArray {
            nums_: nums,
            sums_: sums,
            size_: size,
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.sums_[i as usize / self.size_] += val - self.nums_[i as usize];
        self.nums_[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let mut result: i32 = 0;
        let start: usize = i as usize / self.size_;
        let end: usize = j as usize / self.size_;
        if start == end {
            for index in i..=j {
                result += self.nums_[index as usize];
            }
        } else {
            for index in (i as usize)..((start + 1) * self.size_) {
                result += self.nums_[index];
            }
            for index in (start + 1)..end {
                result += self.sums_[index];
            }
            for index in (end * self.size_)..=(j as usize) {
                result += self.nums_[index];
            }
        }
        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, 5, 6, 8, 9];
    let mut obj: NumArray = NumArray::new(nums);
    println!("{}", obj.sum_range(2, 3));
    obj.update(2, 11);
    println!("{}", obj.sum_range(2, 3));
}
