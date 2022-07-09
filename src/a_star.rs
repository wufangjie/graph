use crate::{Graph, Vertex};

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub};
use utils::Heap;

struct AStarIter<'a, W, F>
where
    W: Clone + Copy + PartialOrd + Add<Output = W>,
    F: Fn(usize) -> W,
{
    edges: &'a Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
    func: F,
}

impl<'a, W, F> AStarIter<'a, W, F>
where
    W: Clone + Copy + PartialOrd + Add<Output = W>,
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
    W: Clone + Copy + PartialOrd + Add<Output = W> + Sub<Output = W>,
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

impl<'a, T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + PartialOrd + Add<Output = W> + Sub<Output = W>,
{
    /// the difference between dijstra and prim's algorithm:
    /// 1. dijstra need to specify a start vertex, while prim needn't
    /// 2. the weight push to the heap: d + w vs w
    /// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
    fn a_star(
        &'a self,
        start: &Vertex<T>,
        func: impl Fn(usize) -> W + 'a,
    ) -> impl Iterator<Item = (W, usize, usize)> + 'a {
        if let Some(&i) = self.v_map.get(start) {
            AStarIter::new(&self.e_lst, i, func)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::from_weighted_edges;
    use crate::Vertex;
    use std::fmt;
    use std::ops::Deref;

    #[derive(PartialEq, Eq, Hash, Clone)]
    struct VData {
        s: &'static str, // symbol
        x: i32,          // NOTE: f64 did not implement Hash, Eq, because NaNs
        y: i32,
    }

    impl fmt::Debug for VData {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.s)
        }
    }

    // impl PartialEq for VData {
    // 	fn eq(&self, other: &Self) -> bool {
    // 	    self.s == other.s
    // 	}
    // }

    // impl Hash for VData {
    // 	fn hash<H: Hasher>(&self, state: &mut H) {
    // 	    self.s.hash(state);
    // 	}
    // }

    impl VData {
        fn new(s: &'static str, x: i32, y: i32) -> Self {
            Self { s, x, y }
        }

        fn dist_to(&self, other: &Self) -> f64 {
            (((self.x - other.x) as f64).powi(2) + ((self.y - other.y) as f64).powi(2)).powf(0.5)
        }
    }

    #[test]
    fn test_a_star() {
        //make_vertices!(a, b, c, d, e, f, g, h, i, j, k, l);
        let a = Vertex::new(VData::new("a", 0, 0));
        let b = Vertex::new(VData::new("b", 1, 0));
        let c = Vertex::new(VData::new("c", 2, 0));
        let d = Vertex::new(VData::new("d", 3, 0));
        let e = Vertex::new(VData::new("e", 0, 1));
        let f = Vertex::new(VData::new("f", 1, 1));
        let g = Vertex::new(VData::new("g", 2, 1));
        let h = Vertex::new(VData::new("h", 3, 1));
        let i = Vertex::new(VData::new("i", 0, 2));
        let j = Vertex::new(VData::new("j", 1, 2));
        let k = Vertex::new(VData::new("k", 2, 2));
        let l = Vertex::new(VData::new("l", 3, 2));

        let g2 = from_weighted_edges!(
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
        //g2.add_rev_edges();

        println!("All distance from {:?}:", &a);
        let calc_dist = |u| g2[u].borrow().deref().dist_to(&l.borrow());
        for (w, u, v) in g2.a_star(&a, calc_dist) {
            println!(
                "distance: {:.4}, to: {:?}, direct from: {:?}",
                w, &g2[u], &g2[v]
            );
        }
    }
}
