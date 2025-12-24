use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        ranges: [Chars;190],
    }

    let mut vec = vec![];
    for i in ranges {
        let mut v = vec![String::new(); 2];
        let mut now = 0;
        for j in i {
            match j {
                '-' => now = 1,
                j => v[now].push(j),
            }
        }

        vec.push((
            v[0].parse::<usize>().unwrap(),
            v[1].parse::<usize>().unwrap(),
        ));
    }

    vec.sort();

    let mut ans: VecDeque<(usize, usize)> = VecDeque::new();
    for (start, end) in vec {
        if let Some((lf, ls)) = ans.pop_back() {
            if start <= ls + 1 {
                ans.push_back((lf, ls.max(end)));
            } else {
                ans.push_back((lf, ls));
                ans.push_back((start, end));
            }
        } else {
            ans.push_back((start, end));
        }
    }

    println!("{}", ans.iter().map(|(f, s)| s - f + 1).sum::<usize>())
}
