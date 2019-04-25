struct MyStack {
    queue_: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
	    queue_: Vec::new(),
	}
    }

    fn push(&mut self, x: i32) {
        self.queue_.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.queue_.pop().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue_.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue_.is_empty()
    }
}

fn main() {
    let mut obj: MyStack = MyStack::new();
    obj.push(1);
    obj.push(2);
    let res1: i32 = obj.pop();
    println!("res1: {}", res1);
    let res2: i32 = obj.top();
    println!("res2: {}", res2);
    let res3: bool = obj.empty();
    println!("res3: {}", res3);
}
