use crate::Graph;
use std::collections::VecDeque;

pub fn bfs<G: Graph>(graph: &G, start: usize) -> impl Iterator<Item = usize> + '_ {
    BfsIter::new(graph, start)
}

/// bfs helper
struct BfsIter<'a, G: Graph> {
    visited: Vec<bool>,
    queue: VecDeque<usize>,
    graph: &'a G,
}

impl<'a, G: Graph> BfsIter<'a, G> {
    fn new(graph: &'a G, start: usize) -> Self {
        let mut visited = vec![false; graph.len()];
        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        Self {
            visited,
            queue,
            graph,
        }
    }
}

impl<'a, G: Graph> Iterator for BfsIter<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.queue.pop_front() {
            for v in self.graph.iter_v_from(u) {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.queue.push_back(v);
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
    fn test_bfs() {
        let g = MakeGraph::scc();
        for v in bfs(&g, 0) {
            dbg!(v);
        }
    }
}
