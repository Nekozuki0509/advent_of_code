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
    g.insert("out".to_string(), vec![]);

    let fft = cnt_ways("fft", &g);
    let dac = cnt_ways("dac", &g);
    let out = cnt_ways("out", &g);
    println!(
        "{}",
        fft["svr"] * dac["fft"] * out["dac"] + dac["svr"] * fft["dac"] * out["fft"]
    );
}

fn cnt_ways(to: &str, g: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut ncounts = BTreeMap::new();
    for i in g.keys() {
        if i == to {
            ncounts.insert(i.to_string(), 1);
        } else {
            ncounts.insert(i.to_string(), 0);
        }
    }

    while ncounts != counts {
        counts = ncounts.clone();
        for i in counts.keys() {
            if i.as_str() == to {
                continue;
            }

            let mut sum = 0;
            for i in &g[i] {
                sum += counts[i];
            }

            ncounts.insert(i.to_string(), sum);
        }
    }

    ncounts
}
