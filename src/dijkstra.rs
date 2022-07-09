use crate::{Graph, Vertex};

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use utils::Heap;

struct DijkstraIter<'a, W>
where
    W: Clone + Copy + PartialOrd + Add<Output = W>,
{
    edges: &'a Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
}

impl<'a, W> DijkstraIter<'a, W>
where
    W: Clone + Copy + PartialOrd + Add<Output = W>,
{
    fn new(edges: &'a Vec<HashMap<usize, W>>, start: usize) -> Self {
        let mut heap = Heap::new();
        for (&v, &w) in &edges[start] {
            heap.push((w, v, start));
        }
        let mut used = vec![false; edges.len()];
        used[start] = true;
        Self { edges, used, heap }
    }
}

impl<'a, W> Iterator for DijkstraIter<'a, W>
where
    W: Clone + Copy + PartialOrd + Add<Output = W>,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((d, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for (&v, &w) in &self.edges[u] {
                    self.heap.push((d + w, v, u));
                }
                return Some((d, u, v));
            }
        }
        None
    }
}

impl<'a, T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + PartialOrd + Add<Output = W>,
{
    /// the difference between dijstra and prim's algorithm:
    /// 1. dijstra need to specify a start vertex, while prim needn't
    /// 2. the weight push to the heap: d + w vs w
    /// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
    fn dijkstra(&'a self, start: &Vertex<T>) -> impl Iterator<Item = (W, usize, usize)> + 'a {
        if let Some(&i) = self.v_map.get(start) {
            DijkstraIter::new(&self.e_lst, i)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_weighted_edges, make_vertices};

    #[test]
    fn test_dijkstra() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let mut g2 = from_weighted_edges!(
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 10);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7)
        );
        g2.add_rev_edges();

        println!("All distance from {}:", &h);
        for (w, u, v) in g2.dijkstra(&h) {
            println!("distance: {}, to: {}, direct from: {}", w, &g2[u], &g2[v]);
        }
    }
}
