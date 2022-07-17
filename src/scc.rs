use crate::{Graph, Vertex, Weight};

impl<T, W: Weight> Graph<T, W> {
    fn scc(&self) -> Vec<Vec<&Vertex<T>>> {
        let n = self.len();
        let seq = self.make_rev_graph().topo_sort_dfs();
        let mut visited = vec![false; n];
        let mut res = vec![];

        for i in seq.into_iter().rev() {
            if !visited[i] {
                res.push(
                    DfsIter::new(self, i, &mut visited)
                        .into_iter()
                        .map(|j| &self[j])
                        .collect(),
                );
            }
        }
        res
    }
}

/// memorized dfs helper
/// NOTE: the difference between scc and dfs's DfsIter
/// visted: Vec<bool> vs &'a mut Vec<bool>
struct DfsIter<'a, T, W: Weight> {
    visited: &'a mut Vec<bool>,
    stack: Vec<usize>,
    graph: &'a Graph<T, W>,
}

impl<'a, T, W: Weight> DfsIter<'a, T, W> {
    fn new(graph: &'a Graph<T, W>, start: usize, visited: &'a mut Vec<bool>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_unweighted_edges, add_vertices};

    #[test]
    fn test_scc() {
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

        dbg!(g1.scc());
    }
}
