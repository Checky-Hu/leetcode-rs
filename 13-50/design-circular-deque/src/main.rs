struct MyCircularDeque {
    start_: usize,
    count_: usize,
    len_: usize,
    v_: Vec<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            start_: 0,
            count_: 0,
            len_: k as usize,
            v_: vec![-1; k as usize],
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.count_ == self.len_ {
            false
        } else {
            self.start_ = if self.start_ == 0 {
                self.len_ - 1
            } else {
                self.start_ - 1
            };
            self.v_[self.start_] = value;
            self.count_ += 1;
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.count_ == self.len_ {
            false
        } else {
            self.v_[(self.start_ + self.count_) % self.len_] = value;
            self.count_ += 1;
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.count_ == 0 {
            false
        } else {
            self.start_ = if self.start_ + 1 == self.len_ {
                0
            } else {
                self.start_ + 1
            };
            self.count_ -= 1;
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.count_ == 0 {
            false
        } else {
            self.count_ -= 1;
            true
        }
    }

    fn get_front(&self) -> i32 {
        if self.count_ == 0 {
            -1
        } else {
            self.v_[self.start_]
        }
    }

    fn get_rear(&self) -> i32 {
        if self.count_ == 0 {
            -1
        } else {
            self.v_[(self.start_ + self.count_ - 1) % self.len_]
        }
    }

    fn is_empty(&self) -> bool {
        self.count_ == 0
    }

    fn is_full(&self) -> bool {
        self.count_ == self.len_
    }
}

fn main() {
    let mut obj: MyCircularDeque = MyCircularDeque::new(3);
    println!("is_empty: {}", obj.is_empty());
    println!("insert_last 1: {}", obj.insert_last(1));
    println!("is_empty: {}", obj.is_empty());
    println!("insert_last 2: {}", obj.insert_last(2));
    println!("insert_front 3: {}", obj.insert_front(3));
    println!("insert_front 4: {}", obj.insert_front(4));
    println!("get_rear: {}", obj.get_rear());
    println!("is_full: {}", obj.is_full());
    println!("delete_last: {}", obj.delete_last());
    println!("insert_front 4: {}", obj.insert_front(4));
    println!("get_front: {}", obj.get_front());
    println!("delete_front: {}", obj.delete_front());
    println!("get_front: {}", obj.get_front());
}
