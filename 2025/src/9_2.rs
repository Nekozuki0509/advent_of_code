use std::collections::VecDeque;

use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        s: [String;496]
    }

    let mut s = s
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .map(|x| (x[0], x[1]))
        .collect_vec();
    s.push(s[0]);

    let (mut l, mut r): (Vec<usize>, Vec<usize>) = s.iter().copied().unzip();
    l = l
        .iter()
        .sorted()
        .copied()
        .flat_map(|x| vec![x - 1, x, x + 1])
        .dedup()
        .collect();
    r = r
        .iter()
        .sorted()
        .copied()
        .flat_map(|x| vec![x - 1, x, x + 1])
        .dedup()
        .collect();

    println!("start edge...");
    let mut g = vec![vec![None; r.len()]; l.len()];
    for (&(i1, j1), &(i2, j2)) in s.iter().tuple_windows() {
        let xmin = l.binary_search(&i1.min(i2)).unwrap();
        let xmax = l.binary_search(&i1.max(i2)).unwrap();
        let ymin = r.binary_search(&j1.min(j2)).unwrap();
        let ymax = r.binary_search(&j1.max(j2)).unwrap();
        for (x, y) in iproduct!(xmin..=xmax, ymin..=ymax) {
            g[x][y] = Some(true);
        }
    }

    println!("start filling...");
    let mut q = VecDeque::new();
    let dist = [(1, 0), (0, 1), (!0, 0), (0, !0)];
    q.push_back((0usize, 0usize));
    while let Some((x, y)) = q.pop_front() {
        for &(dx, dy) in &dist {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if nx < g.len() && ny < g[0].len() && g[nx][ny].is_none() {
                g[nx][ny] = Some(false);
                q.push_back((nx, ny));
            }
        }
    }

    let mut ans = 0;
    for (&(f1, s1), &(f2, s2)) in s.iter().tuple_combinations() {
        println!("checking ({}, {}), ({}, {})", f1, s1, f2, s2);
        let xmin = l.binary_search(&f1.min(f2)).unwrap();
        let xmax = l.binary_search(&f1.max(f2)).unwrap();
        let ymin = r.binary_search(&s1.min(s2)).unwrap();
        let ymax = r.binary_search(&s1.max(s2)).unwrap();
        let mut result = true;
        for (x, y) in iproduct!(xmin..=xmax, ymin..=ymax) {
            if let Some(x) = g[x][y]
                && !x
            {
                result = false;
                break;
            }
        }

        if result {
            ans = ans.max(
                ((f1 as isize - f2 as isize).abs() + 1) * ((s1 as isize - s2 as isize).abs() + 1),
            )
        }
    }

    println!("{ans}")
}
