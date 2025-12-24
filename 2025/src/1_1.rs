use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: [String;4059]
    }

    let s = s
        .into_iter()
        .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse::<isize>().unwrap()))
        .collect_vec();
    let mut now = 50;
    let mut ans = 0;
    for (c, n) in s {
        match c {
            'L' => {
                now -= n;
                now += 100;
                now %= 100;
            }
            _ => {
                now += n;
                now %= 100;
            }
        }

        if now == 0 {
            ans += 1;
        }
    }

    println!("{ans}")
}
