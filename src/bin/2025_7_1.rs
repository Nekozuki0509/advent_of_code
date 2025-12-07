use proconio::{input, marker::Chars};

fn main() {
    input! {d: [Chars;142]}
    let mut now = vec![false; 142];
    now[70] = true;
    let mut ans = 0usize;
    for i in d {
        for (i, &c) in i.iter().enumerate() {
            if c == '^' && now[i] {
                now[i - 1] = true;
                now[i + 1] = true;
                now[i] = false;
                ans += 1;
            }
        }
    }

    println!("{ans}")
}
