use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut ans = 0;
    for _ in 0..153 {
        let mut line = String::new();
        stdin().read_line(&mut line).ok();

        let mut fin = 0;
        let mut btns = vec![];
        let mut dt = true;
        let mut di = 0;
        for i in line.split_whitespace() {
            if dt {
                for i in i.chars() {
                    match i {
                        ']' => dt = false,
                        '[' => {}
                        '#' => {
                            fin |= 1 << di;
                            di += 1;
                        }
                        '.' => di += 1,
                        _ => unreachable!(),
                    }
                }
            } else {
                if i.starts_with('{') {
                    break;
                }

                btns.push(0);
                for i in i
                    .chars()
                    .take(i.len() - 1)
                    .skip(1)
                    .collect::<String>()
                    .split(',')
                {
                    dbg!(i);
                    *btns.last_mut().unwrap() |= 1 << i.parse::<usize>().unwrap();
                }
            }
        }

        let mut q = VecDeque::new();
        let mut dist = vec![!0; 1 << di];
        dist[0] = 0;
        q.push_back(0);
        while let Some(x) = q.pop_front() {
            if x == fin {
                break;
            }

            for &i in &btns {
                if dist[x ^ i] == !0 {
                    dist[x ^ i] = dist[x] + 1;
                    q.push_back(x ^ i);
                }
            }
        }

        ans += dist[fin];
    }

    println!("{ans}")
}
