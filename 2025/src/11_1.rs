use std::{
    collections::{BTreeMap, BTreeSet},
    io::{BufRead, stdin},
};

use itertools::Itertools;

fn main() {
    let mut g = BTreeMap::new();
    for line in stdin().lock().lines().map(|x| x.unwrap()) {
        let lists = line.split_whitespace().collect_vec();
        g.insert(
            lists[0].replace(":", ""),
            lists[1..].iter().map(|x| x.to_string()).collect_vec(),
        );
    }

    let mut ans = 0;
    let mut visited = BTreeSet::new();
    visited.insert("you");
    dfs("you", &g, visited, &mut ans);

    println!("{ans}")
}

fn dfs(now: &str, g: &BTreeMap<String, Vec<String>>, visited: BTreeSet<&str>, ans: &mut usize) {
    if now == "out" {
        *ans += 1;
        return;
    }

    for i in &g[now] {
        if !visited.contains(i.as_str()) {
            let mut visited = visited.clone();
            visited.insert(i.as_str());
            dfs(i.as_str(), g, visited, ans);
        }
    }
}
