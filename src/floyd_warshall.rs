use crate::{Graph, Weight};

impl<T, W: Weight> Graph<T, W> {
    /// can not process negative cycle
    /// O(V^3)
    /// if W is float, we can use const INFINITY to speed up?
    pub fn floyd_warshall(&self) -> Vec<Vec<Option<W>>> {
        let n = self.len();
        let mut dist = vec![vec![None; n]; n];
        for (u, dist_u) in dist.iter_mut().enumerate() {
            dist_u[u] = Some(Default::default());
        }
        for (u, v, w) in self.iter_edges() {
            dist[u][v] = Some(w);
        }
        for i in 0..n {
            for u in 0..n {
                if u == i {
                    // we also can write conditions: v != i, v != u
                    // but I'm not sure how them will influnce the speed
                    continue;
                }
                for v in 0..n {
                    if let Some(d1) = dist[u][i] {
                        if let Some(d2) = dist[i][v] {
                            if let Some(d0) = dist[u][v] {
                                if d1 + d2 < d0 {
                                    dist[u][v] = Some(d1 + d2);
                                }
                            } else {
                                dist[u][v] = Some(d1 + d2);
                            }
                        }
                    }
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices, add_weighted_edges, Vertex};

    #[test]
    fn test_floyd_warshall() {
        // p703. Introduction to Algorithms (Third Edition)
        let mut g4: Graph<(), _> = Graph::new();
        add_vertices!(g4 # a, b, c, d, e);
        add_weighted_edges!(g4 #
	    a: (b, 3), (c, 8), (e, -4);
	    b: (d, 1), (e, 7);
	    c: (b, 4);
	    d: (a, 2), (c, -5);
	    e: (d, 6));
        dbg!(&g4.floyd_warshall());
    }
}
