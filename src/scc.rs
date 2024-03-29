use crate::{Graph, NoWeight, VGraph}; // topo_sort_dfs,
use std::collections::HashMap;

pub fn scc<G: Graph>(graph: &G) -> Vec<Vec<usize>> {
    let n = graph.len();

    // use the simplest VGraph (to reverse) no matter graph is
    let mut lst = vec![HashMap::new(); n];
    for u in 0..n {
        for v in graph.iter_v_from(u) {
            lst[v].insert(u, NoWeight);
        }
    }
    let graph_rev = VGraph::new(lst);

    let seq = graph_rev.topo_sort_dfs();
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
    use crate::MakeGraph;

    #[test]
    fn test_scc() {
        let (g, s_lst) = MakeGraph::scc();
        for (i, part) in g.scc().into_iter().enumerate() {
            print!("\npart {}: ", i);
            for v in part {
                print!("{}, ", s_lst[v]);
            }
            println!();
        }
    }
}
