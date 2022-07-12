use crate::{Graph, Vertex};

use std::hash::Hash;
use std::ops::Add;

impl<'a, T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + PartialOrd + Add<Output = W> + Default + std::fmt::Debug,
{
    /// return (no negative cycle?, dist, from)
    /// unlike dijkstra iter, we will get start vertex in returned result
    /// so we should skip start vertex by hand
    /// TODO: a better way to init distance list: use inf (how?), or Option, or HashMap?
    fn bellman_ford(&self, start: &Vertex<T>) -> (bool, Vec<W>, Vec<usize>) {
        let s = *self
            .v_map
            .get(start)
            .expect("Start vertex not in this graph");
        let n = self.len();

        let mut dist = vec![Default::default(); n];
        let mut from = vec![n; n]; // init n mean from noth
        from[s] = s;

        for _ in 0..n {
            let mut improved = false;
            for u in 0..n {
                for (&v, &w) in &self.e_lst[u] {
                    // so-called relax
                    // the if condition is a bit of hard, a better way
                    if from[u] != n && (from[v] == n || dist[u] + w < dist[v]) {
                        from[v] = u;
                        dist[v] = dist[u] + w;
                        improved = true;
                    }
                }
            }
            if !improved {
                return (true, dist, from);
            }
        }
        (false, dist, from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_weighted_edges, make_vertices};

    #[test]
    fn test_bellman_ford() {
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

        let (s, dist, from) = g2.bellman_ford(&h);
        let n = g2.len();
        println!("All distance from {}:", &h);
        println!("No negative cycle: {}:", s);
        for i in 0..n {
            println!(
                "{}, dist: {}, directly from: {}",
                &g2[i], dist[i], &g2[from[i]]
            )
        }
    }
}
