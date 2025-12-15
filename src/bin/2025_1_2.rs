use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: [String;4059]
    }

    let s = s
        .into_iter()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<isize>().unwrap()))
        .collect_vec();
    let mut now = 50;
    let mut ans = 0;
    let mut checked = false;
    for (c, n) in s {
        dbg!(now);
        match c {
            'L' => {
                now -= n;
                dbg!(now);
                ans -= now / 100;
                now %= 100;
                if now <= 0 && !checked {
                    ans += 1;
                }
                now += 100;
                now %= 100;
                dbg!(ans);
            }
            _ => {
                now += n;
                dbg!(now);
                ans += now / 100;
                now %= 100;
                dbg!(ans);
            }
        }
        checked = now == 0;
    }

    println!("{ans}")
}
