use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars;135]
    }

    let mut ans = 0;
    for (i, j) in iproduct!(0..s.len() as isize, 0..s[0].len() as isize) {
        if s[i as usize][j as usize] != '@' {
            continue;
        }

        let mut cnt = 0;
        for (ni, nj) in iproduct!(-1..=1, -1..=1) {
            if ni == 0 && nj == 0 {
                continue;
            }

            if i + ni >= 0
                && j + nj >= 0
                && i + ni < s.len() as isize
                && j + nj < s[0].len() as isize
                && s[(i + ni) as usize][(j + nj) as usize] != '.'
            {
                cnt += 1;
            }
        }

        if cnt < 4 {
            ans += 1;
        }
    }

    println!("{ans}")
}
