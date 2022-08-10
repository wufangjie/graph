use crate::{Graph, VGraph};

use std::collections::HashMap;

#[allow(clippy::type_complexity)]
pub fn johnson<G: Graph>(graph: &G) -> Vec<(Vec<Option<G::Weight>>, Vec<usize>)> {
    let n = graph.len();
    let h = calc_h(graph).expect("Negative cycle found!");

    let mut lst = Vec::with_capacity(n);
    for u in 0..n {
        let mut map = HashMap::new();
        // no size hint, may a bit slower than using the special structure such as VGraph
        for (v, w) in graph.iter_e_from(u) {
            map.insert(v, w + h[u] - h[v]);
        }
        lst.insert(u, map);
    }

    let g2 = VGraph::new(lst);

    let mut res = Vec::with_capacity(n); // : Vec<(Vec<Option<W>>, Vec<usize>)>
    for i in 0..n {
        let mut dist = vec![None; n];
        let mut from = vec![i; n];
        for (d, u, v) in g2.dijkstra(i) {
            dist[u] = Some(d - h[i] + h[u]);
            from[u] = v;
        }
        res.push((dist, from));
    }
    res
}

/// use modified (faster) bellman ford, to remove negative edges
/// 1. no from (to make a path)
/// 2. no need to add an additional vertex (johnson)
/// 3. the if condition is much easier
/// 4. the super vertex always have an edge to every vertex, so no use to use Option
/// O(V(E+V)logV)
fn calc_h<G: Graph>(graph: &G) -> Option<Vec<G::Weight>> {
    let n = graph.len();
    let mut dist = vec![Default::default(); n];

    for _ in 0..n {
        let mut improved = false;
        for u in 0..n {
            for (v, w) in graph.iter_e_from(u) {
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

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_johnson() {
        let (g, s_lst) = MakeGraph::spn();
        for (u, (dist, from)) in g.johnson().into_iter().enumerate() {
            println!("distances from: {}", s_lst[u]);
            for (i, (od, v)) in dist.into_iter().zip(from.into_iter()).enumerate() {
                if i != u {
                    match od {
                        Some(d) => println!(
                            "to: {}, directly from: {}, distance: {}",
                            s_lst[i], s_lst[v], d
                        ),
                        None => println!(
                            "to: {}, directly from: {}, distance: inf",
                            s_lst[i], s_lst[v]
                        ),
                    }
                }
            }
        }
    }
}
