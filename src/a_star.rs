use crate::{Graph, Vertex, Weight};

use std::collections::HashMap;
use utils::Heap;

impl<'a, T, W: Weight> Graph<T, W> {
    /// the difference between dijstra and prim's algorithm:
    /// 1. dijstra need to specify a start vertex, while prim needn't
    /// 2. the weight push to the heap: d + w vs w
    /// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
    fn a_star(
        &'a self,
        start: &Vertex<T>,
        func: impl Fn(usize) -> W + 'a,
    ) -> impl Iterator<Item = (W, usize, usize)> + 'a {
        if let Some(u) = self.get_index_of(start) {
            AStarIter::new(&self.e_lst, u, func)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

struct AStarIter<'a, W, F>
where
    W: Weight,
    F: Fn(usize) -> W,
{
    edges: &'a Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
    func: F,
}

impl<'a, W, F> AStarIter<'a, W, F>
where
    W: Weight,
    F: Fn(usize) -> W,
{
    fn new(edges: &'a Vec<HashMap<usize, W>>, start: usize, func: F) -> Self {
        let mut heap = Heap::new();
        for (&v, &w) in &edges[start] {
            heap.push((w + func(v), v, start));
        }
        let mut used = vec![false; edges.len()];
        used[start] = true;
        Self {
            edges,
            used,
            heap,
            func,
        }
    }
}

impl<'a, W, F> Iterator for AStarIter<'a, W, F>
where
    W: Weight,
    F: Fn(usize) -> W,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((d, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                let hu = (self.func)(u);
                for (&v, &w) in &self.edges[u] {
                    self.heap.push((d + w - hu + (self.func)(v), v, u));
                }
                return Some((d - hu, u, v));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices_with_data, add_weighted_edges};

    #[test]
    fn test_a_star() {
        let mut g9: Graph<(f64, f64), f64> = Graph::new();

        add_vertices_with_data!(g9 #
            a, (0., 0.);
            b, (1., 0.);
            c, (2., 0.);
            d, (3., 0.);
            e, (0., 1.);
            f, (1., 1.);
            g, (2., 1.);
            h, (3., 1.);
            i, (0., 2.);
            j, (1., 2.);
            k, (2., 2.);
            l, (3., 2.));

        add_weighted_edges!(g9 #
            a: (b, 1.1), (e, 1.0);
            b: (f, 1.0), (c, 2.0);
            c: (g, 2.0), (d, 3.0);
            d: (h, 1.0);
            e: (f, 2.0), (i, 1.0);
            f: (g, 3.0), (j, 2.0);
            g: (h, 1.0), (k, 1.0);
            h: (l, 2.0);
            i: (j, 3.0);
            j: (k, 1.0);
            k: (l, 2.0)
        );

        // dbg!(&g9);

        g9.add_rev_edges();

        println!("All distance from {:?}:", &a);
        let calc_dist = |u| {
            let (x0, y0) = &l.borrow().data;
            let (x1, y1) = g9[u].borrow().data;
            ((x1 - x0).powi(2) + (y1 - y0).powi(2)).powf(0.5)
        };
        for (w, u, v) in g9.a_star(&a, calc_dist) {
            println!(
                "distance: {:.4}, to: {:?}, direct from: {:?}",
                w, &g9[u], &g9[v]
            );
        }
    }
}
