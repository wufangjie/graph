use crate::{Graph, Weight, WeightedEdge};

/// return (no negative cycle?, dist, from)
/// O(VE)
/// The correctness of bellman ford: the count of edges of the shortestpath
/// from one vertex to another is at most V - 1,
/// and we need one more time to check if negative cycle exist
pub fn bellman_ford<W, E, G>(graph: &G, start: usize) -> (bool, Vec<Option<W>>, Vec<usize>)
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let n = graph.len();

    let mut dist = vec![None; n];
    dist[start] = Some(Default::default());
    let mut from = vec![start; n];

    for _ in 0..n {
        let mut improved = false;
        for u in 0..n {
            for e in graph.iter_e_from(u) {
                let v = e.to();
                let w = e.weight();
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

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_bellman_ford() {
        let (g, s_lst) = MakeGraph::mst(true);

        let u = 7;
        let (state, dist, from) = g.bellman_ford(u);
        println!("All distance from {}:", s_lst[u]);
        println!("No negative cycle: {}", state);
        for i in 0..dist.len() {
            println!(
                "to: {}, directly from: {}, distance: {:?}",
                s_lst[i], s_lst[from[i]], dist[i]
            )
        }
    }
}
