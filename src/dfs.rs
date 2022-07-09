use crate::{Graph, Vertex};
use std::hash::Hash;

/// yield vertex as soon as dfs reach it
struct DfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    visited: Vec<bool>,
    stack: Vec<usize>,
    g: &'a Graph<T, W>,
}

impl<'a, T, W> DfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    fn new(g: &'a Graph<T, W>, start: usize) -> Self {
        let n = g.v_lst.len();
        let mut visited = vec![false; n];
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

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    /// NOTE: return index of vertices is promgram friendly
    /// just use graph[index] for human friendly read
    /// + 'a means: all lifetime paremeters of returned type (i.e. DfsIter) outlive 'a
    pub fn dfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = usize> + 'a {
        if let Some(&i) = self.v_map.get(start) {
            DfsIter::new(self, i)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

/// Iterative Deepening Depth-First Search
/// There is really only one situation where IDDFS would be preferable over BFS:
/// when searching a huge acyclic graph
/// (saving a significant amount of memory, with little or no asymptotic slowdown)
struct IddfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    visited: Vec<bool>,
    stack: Vec<(usize, usize)>,
    g: &'a Graph<T, W>,
    start: usize,
    count: usize,
    depth: usize,
}

impl<'a, T, W> IddfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    fn new(g: &'a Graph<T, W>, start: usize) -> Self {
        let n = g.v_lst.len();
        let visited = vec![false; n];
        let stack = vec![];
        Self {
            visited,
            stack,
            g,
            start,
            count: 0,
            depth: 0,
        }
    }
}

impl<'a, T, W> Iterator for IddfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((u, d)) = self.stack.pop() {
                if d > 0 {
                    for &v in self.g.e_lst[u].keys() {
                        self.stack.push((v, d - 1));
                    }
                }
                if !self.visited[u] {
                    self.visited[u] = true;
                    self.count += 1;
                    return Some(u);
                }
            } else if self.count == self.g.len() {
                // NOTE: IMPORTANT
                // we must make sure we can traversal all vertices from `start`
                // otherwise, we will end up with a dead loop
                return None;
            } else {
                self.depth += 1;
                self.stack.push((self.start, self.depth))
            }
        }
    }
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy,
{
    pub fn iddfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = usize> + 'a {
        if let Some(&i) = self.v_map.get(start) {
            IddfsIter::new(self, i)
        } else {
            panic!("Start vertex not in this graph");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_unweighted_edges, make_vertices};

    #[test]
    fn test_dfs() {
        // make unweighted graph
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
        //dbg!(&g1);
        // indexing
        for v in g1.dfs(&a) {
            dbg!(&g1[v]);
        }

        println!("{:->20}", "");
        for v in g1.iddfs(&a) {
            dbg!(&g1[v]);
        }
        println!("{:->20}", "");
    }
}
