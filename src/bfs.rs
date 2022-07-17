use crate::{Graph, Vertex, Weight};
use std::collections::VecDeque;

impl<T, W: Weight> Graph<T, W> {
    pub fn bfs(&self, start: &Vertex<T>) -> impl Iterator<Item = usize> + '_ {
        if let Some(u) = self.get_index_of(start) {
            BfsIter::new(self, u)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

/// bfs helper
struct BfsIter<'a, T, W: Weight> {
    visited: Vec<bool>,
    queue: VecDeque<usize>,
    graph: &'a Graph<T, W>,
}

impl<'a, T, W: Weight> BfsIter<'a, T, W> {
    fn new(graph: &'a Graph<T, W>, start: usize) -> Self {
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

impl<'a, T, W: Weight> Iterator for BfsIter<'a, T, W> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.queue.pop_front() {
            for v in self.graph.iter_vertices_from(u) {
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
    use crate::{add_unweighted_edges, add_vertices};

    #[test]
    fn test_bfs() {
        let mut g1: Graph<(), _> = Graph::new();
        add_vertices!(g1 # a, b, c, d, e, f, g, h, i);
        add_unweighted_edges!(g1 #
            a: b, c;
            b: c, e, i;
            c: d;
            d: a, h;
            e: f;
            f: g;
            g: e, i;
            h: i;
            i: h);

        for v in g1.bfs(&a) {
            dbg!(&g1[v]);
        }
    }
}
