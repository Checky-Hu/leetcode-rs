use std::collections::vec_deque::VecDeque;

struct MyLinkedList {
    v_: VecDeque<i32>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            v_: VecDeque::new(),
        }
    }

    fn get(&self, index: i32) -> i32 {
        if (index as usize) >= self.v_.len() {
            -1
        } else {
            self.v_[index as usize]
        }
    }

    fn add_at_head(&mut self, val: i32) {
        self.v_.push_front(val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.v_.push_back(val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if (index as usize) <= self.v_.len() {
            self.v_.insert(index as usize, val);
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if (index as usize) < self.v_.len() {
            self.v_.remove(index as usize);
        }
    }
}

fn main() {
    let mut obj: MyLinkedList = MyLinkedList::new();
    obj.add_at_head(1);
    obj.add_at_tail(3);
    obj.add_at_index(1, 2);
    println!("get 1: {}", obj.get(1));
    obj.delete_at_index(1);
    println!("get 1: {}", obj.get(1));
}
