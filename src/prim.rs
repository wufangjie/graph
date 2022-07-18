use crate::{Graph, Vertex, Weight};

use std::collections::HashMap;
use utils::Heap;

impl<T, W: Weight> Graph<T, W> {
    /// run prim on directed graph
    /// it is only can be used on the graph,
    /// which exist (a -> b: w) then (b -> a: w),
    /// otherwise we will get bad result
    /// it is faster than prim(), since we needn't to add reverse edges
    /// NOTE: this method used the specail structure of graph
    /// O((E+V)logV)
    fn prim_directed(&self) -> Vec<(W, &Vertex<T>, &Vertex<T>)> {
        PrimIter::new(&self.e_lst)
            .map(|(w, u, v)| (w, &self[u], &self[v]))
            .collect::<Vec<(W, &Vertex<T>, &Vertex<T>)>>()
    }

    fn prim(&self) -> Vec<(W, &Vertex<T>, &Vertex<T>)> {
        let undirected_edges = self.make_undirected_edges();
        PrimIter::new(&undirected_edges)
            .map(|(w, u, v)| (w, &self[u], &self[v]))
            .collect::<Vec<(W, &Vertex<T>, &Vertex<T>)>>()
    }
}

struct PrimIter<'a, W: Weight> {
    edges: &'a Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
}

impl<'a, W: Weight> PrimIter<'a, W> {
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

impl<'a, W: Weight> Iterator for PrimIter<'a, W> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices, add_weighted_edges};

    #[test]
    fn test_prim() {
        let mut g2: Graph<(), _> = Graph::new();
        add_vertices!(g2 # a, b, c, d, e, f, g, h, i);
        add_weighted_edges!(g2 #
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 10);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7));

        dbg!(&g2.prim());

        let mut g3 = g2.clone();
        dbg!(&g3.prim_directed());

        g3.add_rev_edges();
        dbg!(&g3.prim_directed());
    }
}
