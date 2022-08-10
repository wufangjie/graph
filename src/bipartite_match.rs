use crate::Graph;

use std::collections::HashMap;

/// important: make sure the graph is a bipartite
/// this method also can solve: minimum vertex cover problem
/// just select one vertex from each matching:
/// if one vertex has an edge connect (in/out) to a free vertex, select it
/// both of vertices connect to free vertices will not happen (no augmenting path existed)
pub fn bipartite_match<G: Graph>(graph: &G) -> HashMap<usize, usize> {
    let mut matching = HashMap::new();
    for u in 0..graph.len() {
        if !graph.is_empty_from(u) {
            bipartite_augment(graph, &mut matching, u);
        }
    }
    matching
}

/// this implement used dfs to find an augmenting path (using stack),
/// you can use bfs as alternative, (change stack to queue)
fn bipartite_augment<G: Graph>(graph: &G, matching: &mut HashMap<usize, usize>, start: usize) {
    // step1: find augmenting path
    let mut stack = vec![start];
    let mut path = HashMap::new();
    let mut found = usize::MAX;
    while let Some(u) = stack.pop() {
        if graph.is_empty_from(u) {
            // self.e_lst[u].is_empty()
            if let Some(&v) = matching.get(&u) {
                // always can visit once (from matching cancel)
                stack.push(v);
                path.insert(v, u);
            } else {
                found = u;
                break;
            }
        } else {
            for v in graph.iter_v_from(u) {
                // self.e_lst[u].keys() {
                path.entry(v).or_insert_with(|| {
                    stack.push(v); // clippy teach me this
                    u
                });
                // if !path.contains_key(&v) {
                //     stack.push(v);
                //     path.insert(v, u);
                // }
            }
        }
    }

    // step2: augment
    if found != usize::MAX {
        let mut v = found;
        loop {
            let u = *path.get(&v).unwrap(); // it's ok to unwrap
            matching.insert(v, u);
            if u == start {
                return;
            }
            v = *path.get(&u).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_bipartite_match() {
        let (g, s_lst) = MakeGraph::mbm();
        for (v, u) in g.bipartite_match() {
            println!("{} -> {}", s_lst[u], s_lst[v]);
        }
    }
}
