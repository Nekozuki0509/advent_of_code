use std::io::stdin;

use good_lp::*;
use itertools::Itertools;

fn main() {
    let mut ans = 0;
    for t in 0..153 {
        println!("{}\nstarting {}...", "=".repeat(120), t);
        let mut line = String::new();
        stdin().read_line(&mut line).ok();

        let mut fin = vec![];
        let mut btns = vec![];
        for i in line.split_whitespace().skip(1) {
            let j = i
                .chars()
                .take(i.len() - 1)
                .skip(1)
                .collect::<String>()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec();
            if i.starts_with('{') {
                fin = j;
                break;
            }

            btns.push(j);
        }

        let mut vars = variables!();
        let mut x = vec![];
        for _ in 0..btns.len() {
            x.push(vars.add(variable().integer().min(0)));
        }

        let mut constraints = vec![];
        for (counter, &target) in fin.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (i, btn) in btns.iter().enumerate() {
                if btn.contains(&counter) {
                    expr += x[i];
                }
            }

            constraints.push(expr.eq(target as u32));
        }

        let problem = vars
            .minimise(x.iter().copied().sum::<Expression>())
            .using(default_solver)
            .with_all(constraints)
            .solve()
            .unwrap();
        ans += x
            .iter()
            .copied()
            .map(|x| problem.value(x) as usize)
            .sum::<usize>();
    }

    println!("{ans}");
}
