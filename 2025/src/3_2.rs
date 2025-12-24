use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;200]
    }

    let mut ans = 0;
    for i in s {
        let mut v = vec![0usize; 12];
        for (j, &c) in i.iter().enumerate() {
            let n = c.to_string().parse().unwrap();
            let mut flag = false;
            for k in 0..12 {
                if flag {
                    v[k] = 0;
                } else if v[k] < n && i.len() + k >= 12 + j {
                    v[k] = n;
                    flag = true;
                }
            }
        }

        ans += v
            .iter()
            .rev()
            .inspect(|x| print!("{x}"))
            .enumerate()
            .fold(0, |acc, (i, &v)| acc + v * 10usize.pow(i as u32));
        println!();
    }
    println!("{ans}")
}
