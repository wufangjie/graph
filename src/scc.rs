use crate::{Graph, Vertex};
use std::hash::Hash;

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
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

/// NOTE: the difference between scc and dfs's DfsIter
/// visted: Vec<bool> vs &'a mut Vec<bool>
struct DfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    visited: &'a mut Vec<bool>,
    stack: Vec<usize>,
    g: &'a Graph<T, W>,
}

impl<'a, T, W> DfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    fn new(g: &'a Graph<T, W>, start: usize, visited: &'a mut Vec<bool>) -> Self {
        visited[start] = true;
        let stack = vec![start];
        Self { visited, stack, g }
    }
}

impl<'a, T, W> Iterator for DfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.stack.pop() {
            for &v in self.g.e_lst[u].keys() {
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
    use crate::{from_unweighted_edges, make_vertices};

    #[test]
    fn test_scc() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let g1 = from_unweighted_edges!(
            a: b, c;
            b: c, e, i;
            c: d;
            d: a, h;
            e: f;
            f: g;
            g: e, i;
            h: i;
            i: h
        );

        dbg!(g1.scc());
    }
}
