use crate::{Graph, Weight};

use std::collections::HashMap;

impl<T, W: Weight> Graph<T, W> {
    /// important: make sure the graph is a bipartite
    pub fn bipartite_match(&self) -> HashMap<usize, usize> {
        let mut matching = HashMap::new();
        for u in 0..self.len() {
            if !self.e_lst[u].is_empty() {
                self.bipartite_augment(&mut matching, u);
            }
        }
        matching
    }

    /// this implement used dfs to find an augmenting path (using stack),
    /// you can use bfs as alternative, (change stack to queue)
    fn bipartite_augment(&self, matching: &mut HashMap<usize, usize>, start: usize) {
        // step1: find augmenting path
        let mut stack = vec![start];
        let mut path = HashMap::new();
        let mut found = usize::MAX;
        while let Some(u) = stack.pop() {
            if self.e_lst[u].is_empty() {
                if let Some(&v) = matching.get(&u) {
                    // always can visit once (from matching cancel)
                    stack.push(v);
                    path.insert(v, u);
                } else {
                    found = u;
                    break;
                }
            } else {
                for &v in self.e_lst[u].keys() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_unweighted_edges, add_vertices, Vertex};

    #[test]
    fn test_bipartite_match() {
        let mut g6: Graph<(), _> = Graph::new();
        add_vertices!(g6 # x1, x2, x3, x4, x5, x6, y1, y2, y3, y4, y5, y6);
        add_unweighted_edges!(g6 #
            x1: y1, y4, y5;
            x2: y4, y6;
            x3: y1, y3;
            x4: y2;
            x5: y3;
            x6: y1, y3);
        for (&v, &u) in &g6.bipartite_match() {
            println!("{:?} -> {:?}", g6[u], g6[v]);
        }
    }
}
