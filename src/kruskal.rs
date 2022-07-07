use crate::{Graph, Vertex};
use std::cmp::Ordering;
//use std::collections::BinaryHeap;
use std::hash::Hash;
use utils::Heap;

struct Node {
    id: usize,
    parent: usize,
    rank: usize,
}

struct DisjointSet {
    lst: Vec<Node>, // use compact list rather than set
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut lst = Vec::with_capacity(n);
        for i in 0..n {
            lst.push(Node {
                id: i,
                parent: i,
                rank: 0,
            });
        }
        Self { lst }
    }

    fn find_set(&mut self, i: usize) -> usize {
        if self.lst[i].parent != self.lst[i].id {
            self.lst[i].parent = self.find_set(self.lst[i].parent);
        }
        self.lst[i].parent
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find_set(i);
        let j = self.find_set(j);
        if i == j {
            false
        } else {
            match self.lst[i].rank.cmp(&self.lst[j].rank) {
                Ordering::Greater => self.lst[i].parent = j,
                Ordering::Less => self.lst[j].parent = i,
                Ordering::Equal => {
                    self.lst[j].parent = i;
                    self.lst[i].rank += 1;
                }
            }
            true
        }
    }
}

impl<'a, T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default + PartialOrd, // + std::fmt::Debug,
{
    /// NOTE: kruskal can process directed graph (which will be more efficient)
    /// since we need the spanning tree, return Vec rather Iterator
    pub fn kruskal(&self) -> Vec<(W, &Vertex<T>, &Vertex<T>)> {
        let mut heap = Heap::from(
            self.edges()
                .into_iter()
                .map(|(u, v, w)| (w, u, v))
                .collect::<Vec<(W, usize, usize)>>(),
        );

        let n = self.len();
        let mut count = 0;
        let mut ds = DisjointSet::new(n);
        let mut res = Vec::with_capacity(n - 1);
        while let Some((w, u, v)) = heap.pop() {
            if ds.union(u, v) {
                count += 1;
                res.push((w, &self[u], &self[v]));
                if count == n - 1 {
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_weighted_edges, make_vertices};

    #[test]
    fn test_kruskal() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        // TODO: maybe keep alphabet order?
        let g2 = from_weighted_edges!(
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 14);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7)
        );

        dbg!(g2.kruskal());
    }
}
