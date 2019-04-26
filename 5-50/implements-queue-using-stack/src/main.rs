struct MyQueue {
    stack_: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
	    stack_: Vec::new(),
	}
    }

    fn push(&mut self, x: i32) {
        self.stack_.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.stack_.remove(0)
    }

    fn peek(&self) -> i32 {
        *self.stack_.first().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_.is_empty()
    }
}

fn main() {
    let mut obj: MyQueue = MyQueue::new();
    obj.push(1);
    obj.push(2);
    let res1: i32 = obj.pop();
    println!("res1: {}", res1);
    let res2: i32 = obj.peek();
    println!("res2: {}", res2);
    let res3: bool = obj.empty();
    println!("res3: {}", res3);
}
