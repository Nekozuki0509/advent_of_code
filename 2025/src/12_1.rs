use std::{
    collections::BTreeMap,
    io::{BufRead, stdin},
};

use itertools::Itertools;

fn main() {
    let mut flag = false;
    let mut blocks = vec![];
    let mut regions = BTreeMap::new();
    for i in stdin().lock().lines().map(|x| x.unwrap()) {
        if flag {
            if i.is_empty() {
                flag = false;
                continue;
            }

            *blocks.last_mut().unwrap() += i.matches("#").count();
        } else if i.matches(" ").count() == 0 {
            flag = true;
            blocks.push(0);
        } else {
            let (s, t) = i.split_once(":").unwrap();
            regions
                .entry(
                    s.split("x")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
                .or_insert(vec![])
                .push(
                    t.split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec(),
                );
        }
    }

    dbg!(&blocks);

    let mut ans = 0;
    let mut ans_m = 0;
    for ((f, s), t) in regions {
        dbg!(f, s);
        for t in t {
            if t.iter()
                .zip(&blocks)
                .inspect(|&(t, s)| println!("t: {t} s: {s} {}", t * s))
                .map(|(&t, &b)| t * b)
                .sum::<usize>()
                <= f * s
            {
                ans += 1;
                if t.iter().sum::<usize>() * 9 <= f * s {
                    ans_m += 1;
                }
            }
        }
    }

    println!("{ans} {ans_m}");
}
