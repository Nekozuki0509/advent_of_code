use std::{fmt::format, io::stdin};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    let mut s = vec![];
    for _ in 0..4 {
        let mut word = String::new();
        stdin().read_line(&mut word).ok();
        s.push(word.chars().collect_vec());
    }
    let mut nums = vec![vec![]];
    let mut opes = vec![];
    let mut now = 0;
    for i in 0..s[0].len() {
        if s[0][i] == ' ' && s[1][i] == ' ' && s[2][i] == ' ' && s[3][i] == ' ' {
            now += 1;
            nums.push(vec![]);

            input! {o: char}
            match o {
                '+' => opes.push(0),
                _ => opes.push(1),
            }

            continue;
        }

        dbg!(format!("{}{}{}{}", s[0][i], s[1][i], s[2][i], s[3][i]));
        if let Ok(x) = format!("{}{}{}{}", s[0][i], s[1][i], s[2][i], s[3][i])
            .trim()
            .parse::<usize>()
        {
            nums[now].push(x);
        }
    }
    input! {o: char}
    match o {
        '+' => opes.push(0),
        _ => opes.push(1),
    }
    dbg!(&opes);

    println!(
        "{}",
        nums.iter()
            .zip(opes)
            .map(|(x, o)| if o == 0 {
                x.iter().sum::<usize>()
            } else {
                x.iter().product()
            })
            .sum::<usize>()
    );
}
