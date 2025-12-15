use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    s.push(',');
    let mut ans = 0;
    let mut v = vec![String::new(), String::new()];
    let mut now = 0;
    for i in s {
        match i {
            ',' => {
                for i in v[0].parse::<usize>().unwrap()..=v[1].parse().unwrap() {
                    let s = i.to_string();
                    for j in 1..=s.len() / 2 {
                        if s.len() % j != 0 {
                            continue;
                        }

                        let mut flag = true;
                        let tmp = s
                            .chars()
                            .chunks(j)
                            .into_iter()
                            .map(|x| x.collect::<String>())
                            .collect_vec();
                        for (first, second) in tmp.into_iter().tuple_windows() {
                            if first != second {
                                flag = false;
                                break;
                            }
                        }

                        if flag {
                            ans += i;
                            break;
                        }
                    }
                }

                now = 0;
                v = vec![String::new(), String::new()];
            }
            '-' => now = 1,
            i => v[now].push(i),
        }
    }

    println!("{ans}")
}
