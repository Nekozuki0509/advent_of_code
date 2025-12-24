use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        ranges: [Chars;190],
        tests: [usize;1000]
    }

    let mut q = VecDeque::new();
    for i in ranges {
        let mut v = vec![String::new(); 2];
        let mut now = 0;
        for j in i {
            match j {
                '-' => now = 1,
                j => v[now].push(j),
            }
        }

        q.push_back((
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<usize>().unwrap(),
        ));
    }

    let mut ans = 0;
    let mut fin = vec![false; 1000];
    for (start, end) in q {
        for (i, &t) in tests.iter().enumerate() {
            if !fin[i] && start <= t && t <= end {
                ans += 1;
                fin[i] = true;
            }
        }
    }

    println!("{ans}")
}
