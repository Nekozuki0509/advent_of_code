use std::{collections::BTreeSet, mem::swap};

use itertools::{Itertools, iproduct};
use ordered_float::NotNan;
use proconio::input;

#[derive(Clone, Copy)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

impl Pos {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn dist(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

fn main() {
    input! {
        s: [String;1000]
    }

    let s = s
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<f64>().unwrap())
                .collect_vec()
        })
        .map(|x| Pos::new(x[0], x[1], x[2]))
        .collect_vec();

    let mut v = vec![];
    for (i, j) in iproduct!(0..s.len(), 0..s.len()) {
        if i >= j {
            continue;
        }

        v.push((i, j, s[i].dist(s[j])));
    }

    v.sort_by_key(|&(_, _, x)| NotNan::new(x).unwrap());

    let mut uf = UnionFind::new(s.len());
    for &(i, j, x) in v.iter().take(1000) {
        dbg!(i, j, x);
        uf.unite(i, j);
    }

    let mut set = BTreeSet::new();
    let mut v = BTreeSet::new();
    for i in 0..1000 {
        let root = uf.root(i);
        if !set.contains(&root) {
            set.insert(root);
            v.insert(uf.size(root));
        }
    }

    println!("{}", v.iter().rev().take(3).product::<usize>())
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> usize {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return parent;
        }

        if self.siz[parent] < self.siz[child] {
            swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];

        parent
    }

    fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
