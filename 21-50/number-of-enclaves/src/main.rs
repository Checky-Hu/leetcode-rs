use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_enclave(x: usize, y: usize, r: usize, c: usize) -> bool {
        x == 0 || x == r - 1 || y == 0 || y == c - 1
    }

    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        let row: usize = a.len();
        let col: usize = a[0].len();
        let mut visits: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut result: i32 = 0;
        for i in 0..row {
            for j in 0..col {
                if a[i][j] == 0 || visits[i][j] {
                    continue;
                }
                visits[i][j] = true;
                let mut count: i32 = 1;
                let mut is_enclave: bool = Solution::is_enclave(i, j, row, col);
                let mut queue: Vec<(usize, usize)> = Vec::new();
                queue.push((i, j));
                while !queue.is_empty() {
                    let mut next: Vec<(usize, usize)> = Vec::new();
                    for q in queue {
                        if q.0 > 0 && a[q.0 - 1][q.1] == 1 && !visits[q.0 - 1][q.1] {
                            visits[q.0 - 1][q.1] = true;
                            next.push((q.0 - 1, q.1));
                            count += 1;
                            if !is_enclave {
                                is_enclave = Solution::is_enclave(q.0 - 1, q.1, row, col);
                            }
                        }
                        if q.0 + 1 < row && a[q.0 + 1][q.1] == 1 && !visits[q.0 + 1][q.1] {
                            visits[q.0 + 1][q.1] = true;
                            next.push((q.0 + 1, q.1));
                            count += 1;
                            if !is_enclave {
                                is_enclave = Solution::is_enclave(q.0 + 1, q.1, row, col);
                            }
                        }
                        if q.1 > 0 && a[q.0][q.1 - 1] == 1 && !visits[q.0][q.1 - 1] {
                            visits[q.0][q.1 - 1] = true;
                            next.push((q.0, q.1 - 1));
                            count += 1;
                            if !is_enclave {
                                is_enclave = Solution::is_enclave(q.0, q.1 - 1, row, col);
                            }
                        }
                        if q.1 + 1 < col && a[q.0][q.1 + 1] == 1 && !visits[q.0][q.1 + 1] {
                            visits[q.0][q.1 + 1] = true;
                            next.push((q.0, q.1 + 1));
                            count += 1;
                            if !is_enclave {
                                is_enclave = Solution::is_enclave(q.0, q.1 + 1, row, col);
                            }
                        }
                    }
                    queue = next;
                }
                if !is_enclave {
                    result += count;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == n as usize {
                    a.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!("Number of enclaves: {}", Solution::num_enclaves(a));
}
