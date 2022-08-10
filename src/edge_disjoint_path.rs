use crate::Graph;
use std::collections::{HashMap, HashSet};

/// you can get the count of edge disjoint path by: matching[t].len()
/// you also can get one paths solution: see #[test]
pub fn edge_disjoint_path<G: Graph>(
    graph: &G,
    start: usize,
    target: usize,
) -> HashMap<usize, HashSet<usize>> {
    let mut matching = HashMap::new();
    while edge_disjoint_augment(graph, &mut matching, start, target) {}
    matching
}

fn edge_disjoint_augment<G: Graph>(
    graph: &G,
    matching: &mut HashMap<usize, HashSet<usize>>,
    start: usize,
    target: usize,
) -> bool {
    // step1: find augmenting path
    let mut stack = vec![start];
    let mut path = HashMap::new();

    while let Some(u) = stack.pop() {
        for v in graph.iter_v_from(u) {
            //self.e_lst[u].keys() {
            let exist = if let Some(out) = matching.get(&v) {
                out.contains(&u)
            } else {
                false
            };
            if !exist {
                path.entry(v).or_insert_with(|| {
                    stack.push(v);
                    u
                });
                if v == target {
                    break;
                }
            }
        }
        if path.contains_key(&target) {
            break;
        }
        if let Some(out) = matching.get(&u) {
            for &v in out.iter() {
                path.entry(v).or_insert_with(|| {
                    stack.push(v);
                    u
                });
            }
        }
    }

    // step2: augment
    if path.contains_key(&target) {
        let mut v = target;
        while v != start {
            let u = *path.get(&v).unwrap();

            let out = matching.entry(v).or_insert_with(HashSet::new);
            if out.contains(&u) {
                out.remove(&u);
            } else {
                out.insert(u);
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
    use crate::MakeGraph;

    #[test]
    fn test_edge_disjoint_path() {
        let (g, s_lst) = MakeGraph::dp();
        let (s, t) = (0, 6);
        let matching = g.edge_disjoint_path(s, t);
        // dbg!(&matching);

        if !matching.is_empty() {
            let mut used = std::collections::HashSet::new();
            for &v in matching.get(&t).unwrap() {
                let mut res = vec![s_lst[t], s_lst[v]];
                let mut v = v;
                loop {
                    for &u in matching.get(&v).unwrap() {
                        if !used.contains(&(v, u)) {
                            res.push(s_lst[u]);
                            used.insert((v, u));
                            v = u;
                            break;
                        }
                    }
                    if v == s {
                        break;
                    }
                }
                dbg!(res);
            }
        }
    }
}
