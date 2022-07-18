use crate::{Graph, Vertex, Weight};

impl<T, W: Weight> Graph<T, W> {
    /// return (no negative cycle?, dist, from)
    /// unlike dijkstra iter, we will get start vertex in returned result
    /// so we should skip start vertex by hand
    /// TODO: a better way to init distance list: use inf (how?), or Option, or HashMap?
    /// O(VE)
    pub fn bellman_ford(&self, start: &Vertex<T>) -> (bool, Vec<Option<W>>, Vec<usize>) {
        let s = self
            .get_index_of(start)
            .expect("Start vertex not in this graph");
        let n = self.len();

        let mut dist = vec![None; n];
        dist[s] = Some(Default::default());
        let mut from = vec![s; n];

        for _ in 0..n {
            let mut improved = false;
            for u in 0..n {
                for (&v, &w) in &self.e_lst[u] {
                    // so-called relax
                    if let Some(d) = dist[u] {
                        let can_improve = match dist[v] {
                            None => true,
                            Some(d0) => d + w < d0,
                        };
                        if can_improve {
                            from[v] = u;
                            dist[v] = Some(d + w);
                            improved = true;
                        }
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
    use crate::{add_vertices, add_weighted_edges};

    #[test]
    fn test_bellman_ford() {
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
        g2.add_rev_edges();

        let (s, dist, from) = g2.bellman_ford(&h);
        println!("All distance from {}:", &h);
        println!("No negative cycle: {}:", s);
        for i in 0..g2.len() {
            println!(
                "{}, dist: {:?}, directly from: {}",
                &g2[i], dist[i], &g2[from[i]]
            )
        }
    }
}
