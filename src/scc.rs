use crate::{topo_sort_dfs, Edge, Graph, NoWeight, VGraph};
use std::collections::HashMap;

fn scc<G: Graph>(graph: &G) -> Vec<Vec<usize>> {
    let n = graph.len();

    let mut lst = vec![HashMap::new(); n];
    for e in graph.iter_e_all() {
        lst[e.to()].insert(e.from(), NoWeight);
    }
    let graph_rev = VGraph::new(lst);

    let seq = topo_sort_dfs(&graph_rev);
    let mut visited = vec![false; n];
    let mut res = vec![];

    for i in seq.into_iter().rev() {
        if !visited[i] {
            res.push(DfsIter::new(graph, i, &mut visited).into_iter().collect());
        }
    }
    res
}

/// memorized dfs helper
/// NOTE: the difference between scc and dfs's DfsIter
/// visted: Vec<bool> vs &'a mut Vec<bool>
struct DfsIter<'a, G: Graph> {
    visited: &'a mut Vec<bool>,
    stack: Vec<usize>,
    graph: &'a G,
}

impl<'a, G: Graph> DfsIter<'a, G> {
    fn new(graph: &'a G, start: usize, visited: &'a mut Vec<bool>) -> Self {
        visited[start] = true;
        let stack = vec![start];
        Self {
            visited,
            stack,
            graph,
        }
    }
}

impl<'a, G: Graph> Iterator for DfsIter<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.stack.pop() {
            for v in self.graph.iter_v_from(u) {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                }
            }
            Some(u)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_scc() {
        let g = MakeGraph::scc();
        dbg!(scc(&g));
    }
}
