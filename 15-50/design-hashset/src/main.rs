struct MyHashSet {
    v_: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet {
	    v_: vec![false; 1000001],
	}
    }

    fn add(&mut self, key: i32) {
        self.v_[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.v_[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.v_[key as usize]
    }
}

fn main() {
    let mut obj: MyHashSet = MyHashSet::new();
    obj.add(1);
    obj.add(2);
    let res1: bool = obj.contains(2);
    println!("res1: {}", res1);
    obj.remove(2);
    let res2: bool = obj.contains(2);
    println!("res2: {}", res2);
    let res3: bool = obj.contains(1);
    println!("res3: {}", res3);
}
