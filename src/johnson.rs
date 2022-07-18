use crate::{DijkstraIter, Graph, Weight};

use std::collections::HashMap;

impl<'a, T, W: Weight> Graph<T, W> {
    /// use modified (faster) bellman ford, to remove negative edges
    /// 1. no from (to make a path)
    /// 2. no need to add an additional vertex (johnson)
    /// 3. the if condition is much easier
    /// 4. the super vertex always have an edge to every vertex, so no use to use Option
    /// O(V(E+V)logV)
    fn calc_h(&self) -> Option<Vec<W>> {
        let n = self.len();
        let mut dist = vec![Default::default(); n];

        for _ in 0..n {
            let mut improved = false;
            for u in 0..n {
                for (&v, &w) in &self.e_lst[u] {
                    if dist[u] + w < dist[v] {
                        dist[v] = dist[u] + w;
                        improved = true;
                    }
                }
            }
            if !improved {
                return Some(dist);
            }
        }
        None
    }

    pub fn johnson(&self) -> Vec<(Vec<Option<W>>, Vec<usize>)> {
        let n = self.len();
        let h = self.calc_h().expect("Negative cycle found!");

        let mut e_lst = Vec::with_capacity(n);
        for (u, dct) in self.e_lst.iter().enumerate() {
            let mut map = HashMap::with_capacity(dct.len());
            for (&v, &w) in dct {
                map.insert(v, w + h[u] - h[v]);
            }
            e_lst.push(map);
        }

        let mut res: Vec<(Vec<Option<W>>, Vec<usize>)> = Vec::with_capacity(n);
        for i in 0..n {
            let mut dist = vec![None; n];
            let mut from = vec![i; n];
            for (d, u, v) in DijkstraIter::new(&e_lst, i) {
                dist[u] = Some(d - h[i] + h[u]);
                from[u] = v;
            }
            res.push((dist, from));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices, add_weighted_edges, Vertex};

    #[test]
    fn test_johnson() {
        // p703. Introduction to Algorithms (Third Edition)
        let mut g4: Graph<(), _> = Graph::new();
        add_vertices!(g4 # a, b, c, d, e);
        add_weighted_edges!(g4 #
	    a: (b, 3), (c, 8), (e, -4);
	    b: (d, 1), (e, 7);
	    c: (b, 4);
	    d: (a, 2), (c, -5);
	    e: (d, 6));
        for (dist, from) in g4.johnson() {
            dbg!(&dist);
            dbg!(&from);
        }
    }
}
