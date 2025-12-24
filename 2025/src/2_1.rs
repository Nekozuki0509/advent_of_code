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
                    if s.len() % 2 == 1 {
                        continue;
                    }

                    let (first, second) = s.split_at(s.len() / 2);
                    if first == second {
                        ans += i;
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
