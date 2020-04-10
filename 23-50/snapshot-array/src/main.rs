struct SnapshotArray {
    // [[(snap_id, val)], ...]
    v_: Vec<Vec<(i32, i32)>>,
    snap_id_: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            v_: vec![vec![(0, 0)]; length as usize],
            snap_id_: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.v_[index as usize].push((self.snap_id_, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id_ += 1;
        self.snap_id_ - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let temp: &Vec<(i32, i32)> = &self.v_[index as usize];
        let mut pos: usize = temp.len() - 1;
        for (i, v) in temp.iter().enumerate() {
            if v.0 > snap_id {
                pos = i - 1;
                break;
            }
        }
        temp[pos].1
    }
}

fn main() {
    let mut obj: SnapshotArray = SnapshotArray::new(3);
    obj.set(0, 5);
    println!("snap: {}", obj.snap());
    obj.set(0, 6);
    println!("get(0, 0): {}", obj.get(0, 0));
}
