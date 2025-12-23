use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: [String;496]
    }

    let s = s
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect_vec()
        })
        .map(|x| (x[0], x[1]))
        .collect_vec();

    let mut ans = 0;
    for (&(f1, s1), &(f2, s2)) in s.iter().tuple_combinations() {
        ans = ans.max(((f1 - f2).abs() + 1) * ((s1 - s2).abs() + 1))
    }

    println!("{ans}")
}
