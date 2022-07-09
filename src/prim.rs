use crate::{Graph, Vertex};

use std::collections::HashMap;
use std::hash::Hash;
use utils::Heap;

struct PrimIter<'a, W>
where
    W: Clone + Copy + PartialOrd,
{
    edges: &'a Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
}

impl<'a, W> PrimIter<'a, W>
where
    W: Clone + Copy + PartialOrd,
{
    fn new(edges: &'a Vec<HashMap<usize, W>>) -> Self {
        let start = 0;
        let mut heap = Heap::new();
        for (&v, &w) in &edges[start] {
            heap.push((w, v, start));
        }
        let mut used = vec![false; edges.len()];
        used[start] = true;
        Self { edges, used, heap }
    }
}

impl<'a, W> Iterator for PrimIter<'a, W>
where
    W: Clone + Copy + PartialOrd,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((w, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for (&v, &w) in &self.edges[u] {
                    self.heap.push((w, v, u));
                }
                return Some((w, u, v)); // NOTE: v, w is ok
            }
        }
        None
    }
}

impl<'a, T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + PartialOrd, // + std::fmt::Debug,
{
    /// run prim on directed graph
    /// it is only can be used on the graph,
    /// which exist (a -> b: w) then (b -> a: w),
    /// otherwise we will get bad result
    /// it is faster than prim(), since we needn't to add reverse edges
    fn prim_directed(&self) -> Vec<(W, &Vertex<T>, &Vertex<T>)> {
        PrimIter::new(&self.e_lst)
            .map(|(w, u, v)| (w, &self[u], &self[v]))
            .collect::<Vec<(W, &Vertex<T>, &Vertex<T>)>>()
    }

    fn prim(&self) -> Vec<(W, &Vertex<T>, &Vertex<T>)> {
        let mut edges = self.get_rev_edges();
        for (u, dct) in self.e_lst.iter().enumerate() {
            for (&v, &w) in dct {
                edges[u].insert(v, w);
            }
        }
        PrimIter::new(&edges)
            .map(|(w, u, v)| (w, &self[u], &self[v]))
            .collect::<Vec<(W, &Vertex<T>, &Vertex<T>)>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_weighted_edges, make_vertices};

    #[test]
    fn test_prim() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let g2 = from_weighted_edges!(
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 10);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7)
        );

        dbg!(&g2.prim());

        let mut g3 = g2.clone();
        dbg!(&g3.prim_directed());

        g3.add_rev_edges();
        dbg!(&g3.prim_directed());
    }
}
