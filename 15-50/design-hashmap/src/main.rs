struct MyHashMap {
    v_: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
	    v_: vec![-1; 1000001],
	}
    }

    fn put(&mut self, key: i32, value: i32) {
        self.v_[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.v_[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.v_[key as usize] = -1;
    }
}

fn main() {
    let mut obj: MyHashMap = MyHashMap::new();
    obj.put(1, 100);
    obj.put(2, 101);
    let res1: i32 = obj.get(2);
    println!("res1: {}", res1);
    obj.remove(2);
    let res2: i32 = obj.get(2);
    println!("res2: {}", res2);
    let res3: i32 = obj.get(1);
    println!("res3: {}", res3);
}
