use std::io::{Read, stdin};

use itertools::Itertools;

fn main() {
    let mut word = String::new();
    stdin().read_to_string(&mut word).ok();
    let words = word.split_whitespace().collect_vec();

    let mut nums = vec![];
    let mut opes = vec![];

    for i in words {
        if let Ok(x) = i.parse::<usize>() {
            nums.push(x);
        } else {
            match i {
                "+" => opes.push(0),
                _ => opes.push(1),
            }
        }
    }

    let nums = nums.chunks(opes.len()).collect_vec();
    let mut v = vec![vec![]; opes.len()];
    for i in 0..opes.len() {
        for j in 0..nums.len() {
            v[i].push(nums[j][i]);
        }
    }

    println!(
        "{}",
        v.iter()
            .zip(opes)
            .map(|(x, o)| if o == 0 {
                x.iter().sum::<usize>()
            } else {
                x.iter().product()
            })
            .sum::<usize>()
    );
}
