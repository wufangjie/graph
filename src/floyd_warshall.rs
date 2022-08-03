use crate::{Graph, Weight, WeightedEdge};

/// can not process negative cycle
/// O(V^3)
/// if W is float, we can use const INFINITY to speed up?
pub fn floyd_warshall<W, E, G>(graph: &G) -> Vec<Vec<Option<W>>>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let n = graph.len();
    let mut dist = vec![vec![None; n]; n];
    for (u, dist_u) in dist.iter_mut().enumerate() {
        dist_u[u] = Some(Default::default());
    }
    for e in graph.iter_e_all() {
        dist[e.from()][e.to()] = Some(e.weight());
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

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_floyd_warshall() {
        let (g, s_lst) = MakeGraph::spn();
        for (u, row) in g.floyd_warshall().into_iter().enumerate() {
            println!("distances from: {}", s_lst[u]);
            for (v, od) in row.into_iter().enumerate() {
                if v != u {
                    match od {
                        Some(d) => println!("to: {}, distance: {}", s_lst[v], d),
                        None => println!("to: {}, distance: inf", s_lst[v]),
                    }
                }
            }
        }
    }
}
