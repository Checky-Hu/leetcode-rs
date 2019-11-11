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

    pub fn quick_sort_descend(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: i32 = nums[left];
        while i < j {
            while i < j && nums[j] <= temp {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j && nums[i] >= temp {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = temp;
        if left + 1 < i {
            quick_sort_descend(nums, left, i - 1);
        }
        quick_sort_descend(nums, i + 1, right);
    }
}

pub mod qsvec {
    pub fn quick_sort(nums: &mut Vec<Vec<i32>>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: Vec<i32> = nums[left].clone();
        while i < j {
            while i < j && nums[j][0] >= temp[0] {
                j -= 1;
            }
            nums[i][0] = nums[j][0];
            nums[i][1] = nums[j][1];
            while i < j && nums[i][0] <= temp[0] {
                i += 1;
            }
            nums[j][0] = nums[i][0];
            nums[j][1] = nums[i][1];
        }
        nums[i][0] = temp[0];
        nums[i][1] = temp[1];
        if left + 1 < i {
            quick_sort(nums, left, i - 1);
        }
        quick_sort(nums, i + 1, right);
    }
}

