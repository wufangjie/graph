use crate::{Graph, VGraph, Weight, WeightedEdge};

use std::collections::HashMap;

pub fn johnson<W, E, G>(graph: &G) -> Vec<(Vec<Option<W>>, Vec<usize>)>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let n = graph.len();
    let h = calc_h(graph).expect("Negative cycle found!");

    let mut lst = Vec::with_capacity(n);
    for u in 0..n {
        let mut map = HashMap::new();
        // no size hint, may a bit slower than using the special structure such as VGraph
        for e in graph.iter_e_from(u) {
            let v = e.to();
            map.insert(v, e.weight() + h[u] - h[v]);
        }
        lst.insert(u, map);
    }

    let g2 = VGraph::new(lst);

    // for (u, dct) in self.e_lst.iter().enumerate() {
    //     let mut map = HashMap::with_capacity(dct.len());
    //     for (&v, &w) in dct {
    //         map.insert(v, w + h[u] - h[v]);
    //     }
    //     e_lst.push(map);
    // }

    let mut res: Vec<(Vec<Option<W>>, Vec<usize>)> = Vec::with_capacity(n);
    for i in 0..n {
        let mut dist = vec![None; n];
        let mut from = vec![i; n];
        for (d, u, v) in g2.dijkstra(i) {
            //for (d, u, v) in DijkstraIter::new(&e_lst, i) {
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
fn calc_h<W, E, G>(graph: &G) -> Option<Vec<W>>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let n = graph.len();
    let mut dist = vec![Default::default(); n];

    for _ in 0..n {
        let mut improved = false;
        for u in 0..n {
            for e in graph.iter_e_from(u) {
                let v = e.to();
                let w = e.weight();
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
