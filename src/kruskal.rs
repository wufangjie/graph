use crate::{Graph, Weight, WeightedEdge};
use std::cmp::Ordering;
//use std::collections::BinaryHeap;
use utils::Heap;

/// NOTE: kruskal can process directed graph (which will be more efficient)
/// since we need the spanning tree, return Vec rather Iterator
/// O(ElogV)
pub fn kruskal<W, E, G>(graph: &G) -> Vec<(W, usize, usize)>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let mut heap = Heap::from(
        graph
            .iter_e_all()
            .map(|e| (e.weight(), e.from(), e.to()))
            .collect::<Vec<(W, usize, usize)>>(),
    );

    let n = graph.len();
    let mut count = 0;
    let mut ds = DisjointSet::new(n);
    let mut res = Vec::with_capacity(n - 1);
    while let Some((w, u, v)) = heap.pop() {
        if ds.union(u, v) {
            count += 1;
            res.push((w, u, v));
            if count == n - 1 {
                break;
            }
        }
    }
    res
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_kruskal() {
        let g = MakeGraph::mst(false);
        let res = kruskal(&g);
        assert_eq!(res.iter().map(|(w, _u, _v)| *w).sum::<i32>(), 37);
        dbg!(res);
    }
}
