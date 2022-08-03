use crate::{Graph, Weight, WeightedEdge};
use std::collections::HashMap;

/// you can get the count of vertex disjoint path using: last_but_t.len()
/// you can get one paths solution:
/// 1. last_but_t keeps all vertice directly to t
/// 2. matching keeps all sub paths (backward, (k, v) in it means edge v -> k)
pub fn vertex_disjoint_path<W, E, G>(
    graph: &G,
    start: usize,
    target: usize,
) -> (HashMap<usize, usize>, Vec<usize>)
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    let mut matching = HashMap::new();
    let mut last_but_t = vec![];
    while vertex_disjoint_augment(graph, &mut matching, &mut last_but_t, start, target) {}
    (matching, last_but_t)
}

fn vertex_disjoint_augment<W, E, G>(
    graph: &G,
    matching: &mut HashMap<usize, usize>,
    last_but_t: &mut Vec<usize>,
    start: usize,
    target: usize,
) -> bool
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    // step1: find augmenting path
    let mut stack = vec![start];
    let mut path = HashMap::new();
    let mut found = usize::MAX;

    while let Some(u) = stack.pop() {
        if let Some(&v) = matching.get(&u) {
            // every time we can cancel a previous matching
            // NOTE: we always need to update path (v <- u),
            // no matter v has in path or not
            // we may put v into stack twice, but it's ok
            path.insert(v, u);
            stack.push(v);

            // if vertex(u) is in matching, but isn't from cancellation
            // we can not add u's outdegrees
            // NOTE: s will always not in matching, so u != s, unwrap is safe
            if matching.get(path.get(&u).unwrap()) != Some(&u) {
                continue;
            }
        }
        for v in graph.iter_v_from(u) {
            if v == target {
                found = u;
                break;
            }
            path.entry(v).or_insert_with(|| {
                stack.push(v);
                u
            });
        }
    }

    // step2: augment
    if found != usize::MAX {
        last_but_t.push(found);
        let mut v = found;
        while v != start {
            let u = *path.get(&v).unwrap();
            // NOTE: it's important to remove matching when cancel
            if matching.get(&u) == Some(&v) {
                matching.remove(&u);
            } else {
                matching.insert(v, u);
            }
            v = u;
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_vertex_disjoint_path() {
        let (g, s_lst) = MakeGraph::dp();

        let (s, t) = (0, 6);
        let (matching, last_but_t) = g.vertex_disjoint_path(s, t);
        dbg!(&last_but_t);
        dbg!(&matching);

        for mut i in last_but_t {
            let mut res = vec![s_lst[t]];
            loop {
                let v = i;
                res.push(s_lst[i]);
                if v == s {
                    break;
                } else {
                    i = *matching.get(&i).unwrap();
                }
            }
            dbg!(res);
        }
    }
}
