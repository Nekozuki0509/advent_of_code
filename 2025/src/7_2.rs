use proconio::{input, marker::Chars};

fn main() {
    input! {d: [Chars;142]}
    let mut now = vec![0; 142];
    now[70] = 1;
    let mut ans = 1usize;
    for i in d {
        for (i, &c) in i.iter().enumerate() {
            if c == '^' && now[i] > 0 {
                now[i - 1] += now[i];
                now[i + 1] += now[i];
                ans += now[i];
                now[i] = 0;
            }
        }
    }

    println!("{}", ans);
}
