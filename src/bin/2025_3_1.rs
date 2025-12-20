use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;200]
    }

    let mut ans = 0;
    for i in s {
        let mut first = 0;
        let mut second = 0;
        for (j, &c) in i.iter().enumerate() {
            let n = c.to_string().parse().unwrap();
            if first < n && j + 1 != i.len() {
                first = n;
                second = 0;
            } else if second < n {
                second = n;
            }
        }

        ans += first * 10 + second;
    }
    println!("{ans}")
}
