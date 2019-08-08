use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_valid_max_minute(m: i32, n: i32) -> i32 {
        let tmp_m1: i32 = 10 * m + n;
        let tmp_m2: i32 = 10 * n + m;
        if tmp_m1 < 60 {
            if tmp_m2 < 60 {
                if tmp_m1 >= tmp_m2 {
                    tmp_m1
                } else {
                    tmp_m2
                }
            } else {
                tmp_m1
            }
        } else {
            if tmp_m2 < 60 {
                tmp_m2
            } else {
                -1
            }
        }
    }

    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut hour: i32 = -1;
        let mut minute: i32 = -1;
        let mut i: usize = 0;
        while i < 3 {
            let mut j: usize = i + 1;
            while j < 4 {
                let mut m: i32 = -1;
                let mut n: i32 = -1;
                for k in 0..4 {
                    if k != i && k != j {
                        if m >= 0 {
                            n = a[k];
                        } else {
                            m = a[k];
                        }
                    }
                }
                let h1: i32 = 10 * a[i] + a[j];
                let mut m1: i32 = -1;
                if h1 < 24 {
                    m1 = Solution::get_valid_max_minute(m, n);
                }
                let h2: i32 = 10 * a[j] + a[i];
                let mut m2: i32 = -1;
                if h2 < 24 {
                    m2 = Solution::get_valid_max_minute(m, n);
                }
                let (h, m) = if h1 < 24 && m1 > -1 {
                    if h2 < 24 && m2 > -1 {
                        if h1 < h2 {
                            (h2, m2)
                        } else if h1 == h2 {
                            if m1 <= m2 {
                                (h2, m2)
                            } else {
                                (h1, m1)
                            }
                        } else {
                            (h1, m1)
                        }
                    } else {
                        (h1, m1)
                    }
                } else {
                    if h2 < 24 && m2 > -1 {
                        (h2, m2)
                    } else {
                        (-1, -1)
                    }
                };
                if h > -1 && m > -1 {
                    if hour < h {
                        hour = h;
                        minute = m;
                    } else if hour == h {
                        if minute < m {
                            minute = m;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
        }
        if hour > -1 && minute > -1 {
            let mut result: String = String::new();
            if hour < 10 {
                result.push('0');
            }
            result.push_str(&hour.to_string());
            result.push(':');
            if minute < 10 {
                result.push('0');
            }
            result.push_str(&minute.to_string());
            result
        } else {
            String::new()
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
            if ret == 4 {
                break;
            }
        }
    }

    if 4 != ret {
        println!("Require at least four parameters.");
        return;
    }

    println!("Time: {}", Solution::largest_time_from_digits(a));
}

