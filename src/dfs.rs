use crate::{Graph, Vertex, Weight};

impl<T, W: Weight> Graph<T, W> {
    /// NOTE: return index of vertices is promgram friendly
    /// just use graph[index] for human friendly read
    pub fn dfs(&self, start: &Vertex<T>) -> impl Iterator<Item = usize> + '_ {
        if let Some(u) = self.get_index_of(start) {
            DfsIter::new(self, u)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

impl<T, W: Weight> Graph<T, W> {
    pub fn iddfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = usize> + 'a {
        if let Some(u) = self.get_index_of(start) {
            IddfsIter::new(self, u)
        } else {
            panic!("Start vertex not in this graph");
        }
    }
}

/// dfs helper
/// yield vertex as soon as dfs reach it
struct DfsIter<'a, T, W: Weight> {
    visited: Vec<bool>,
    stack: Vec<usize>,
    graph: &'a Graph<T, W>,
}

impl<'a, T, W: Weight> DfsIter<'a, T, W> {
    fn new(graph: &'a Graph<T, W>, start: usize) -> Self {
        let mut visited = vec![false; graph.len()];
        visited[start] = true;
        let stack = vec![start];
        Self {
            visited,
            stack,
            graph,
        }
    }
}

impl<'a, T, W: Weight> Iterator for DfsIter<'a, T, W> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.stack.pop() {
            for v in self.graph.iter_vertices_from(u) {
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

/// iddfs helper
/// Iterative Deepening Depth-First Search
/// There is really only one situation where IDDFS would be preferable over BFS:
/// when searching a huge acyclic graph
/// (saving a significant amount of memory, with little or no asymptotic slowdown)
struct IddfsIter<'a, T, W: Weight> {
    visited: Vec<bool>,
    stack: Vec<(usize, usize)>,
    graph: &'a Graph<T, W>,
    start: usize,
    count: usize,
    depth: usize,
}

impl<'a, T, W: Weight> IddfsIter<'a, T, W> {
    /// maybe we should provide a parameter to set init depth?
    fn new(graph: &'a Graph<T, W>, start: usize) -> Self {
        let visited = vec![false; graph.len()];
        let stack = vec![];
        Self {
            visited,
            stack,
            graph,
            start,
            count: 0,
            depth: 0,
        }
    }
}

impl<'a, T, W: Weight> Iterator for IddfsIter<'a, T, W> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: depth's upper bound: the number of vertices
        while self.depth < self.graph.len() {
            if let Some((u, d)) = self.stack.pop() {
                if d > 0 {
                    for v in self.graph.iter_vertices_from(u) {
                        self.stack.push((v, d - 1));
                    }
                }
                if !self.visited[u] {
                    self.visited[u] = true;
                    self.count += 1;
                    return Some(u);
                }
            } else if self.count == self.graph.len() {
                break;
            } else {
                self.depth += 1;
                self.stack.push((self.start, self.depth))
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_unweighted_edges, add_vertices};

    #[test]
    fn test_dfs() {
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

        for v in g1.dfs(&a) {
            dbg!(&g1[v]);
        }

        println!("{:->20}", "");

        for v in g1.iddfs(&a) {
            dbg!(&g1[v]);
        }
    }
}
