#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod qsi32 {
    pub fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: i32 = nums[left];
        while i < j {
            while i < j && nums[j] >= temp {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j && nums[i] <= temp {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = temp;
        if left + 1 < i {
            quick_sort(nums, left, i - 1);
        }
        quick_sort(nums, i + 1, right);
    }
}
