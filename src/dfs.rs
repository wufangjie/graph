use crate::{Graph, Vertex};
use std::hash::Hash;

/// yield vertex as soon as dfs reach it
struct DfsFirst<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    visited: Vec<bool>,
    stack: Vec<usize>,
    g: &'a Graph<T, W>,
}

impl<'a, T, W> DfsFirst<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn new(g: &'a Graph<T, W>, start: usize) -> Self {
        let n = g.v_lst.len();
        let mut visited = vec![false; n];
        visited[start] = true;
        let stack = vec![start];
        Self { visited, stack, g }
    }
}

impl<'a, T, W> Iterator for DfsFirst<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    type Item = &'a Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.stack.pop() {
            for &v in self.g.e_lst[u].keys() {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                }
            }
            Some(&self.g[u])
        } else {
            None
        }
    }
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn dfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = &'a Vertex<T>> {
        // + 'a
        if let Some(&i) = self.v_map.get(start) {
            DfsFirst::new(self, i)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

/// yield vertex after all it's out-degree vertices been visited
/// this implement can processs graph which are not DAG without a dead loop
/// it is useful to topological sort and scc
struct DfsLast<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    visited: Vec<i8>, // actually it is visit times
    stack: Vec<usize>,
    g: &'a Graph<T, W>,
}

impl<'a, T, W> DfsLast<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn new(g: &'a Graph<T, W>, start: usize) -> Self {
        let n = g.v_lst.len();
        let mut visited = vec![0; n];
        visited[start] = 1;
        let stack = vec![start];
        Self { visited, stack, g }
    }
}

impl<'a, T, W> Iterator for DfsLast<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    type Item = &'a Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(u) = self.stack.pop() {
                if self.visited[u] == 2 {
                    return Some(&self.g[u]);
                } else {
                    self.stack.push(u);
                    self.visited[u] += 1;
                    for &v in self.g.e_lst[u].keys() {
                        if self.visited[v] == 0 {
                            self.visited[v] = 1;
                            self.stack.push(v);
                        }
                    }
                }
            } else {
                return None;
            }
        }
    }
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn dfs_last<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = &'a Vertex<T>> {
        if let Some(&i) = self.v_map.get(start) {
            DfsLast::new(self, i)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

/// Iterative Deepening Depth-First Search
/// There is really only one situation where IDDFS would be preferable over BFS:
/// when searching a huge acyclic graph
/// (saving a significant amount of memory, with little or no asymptotic slowdown)
/// TODO: why dfs save memory than bfs (recursive dfs rather than stack based dfs?)
struct Iddfs<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    visited: Vec<bool>,
    stack: Vec<(usize, usize)>,
    g: &'a Graph<T, W>,
    start: usize,
    count: usize, // all j < i have been processed
    depth: usize,
}

impl<'a, T, W> Iddfs<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
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

impl<'a, T, W> Iterator for Iddfs<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    type Item = &'a Vertex<T>;

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
                    return Some(&self.g[u]);
                }
            } else {
                // NOTE: we must make sure we can traversal the graph from start
                // otherwise, we will end up with dead loop
                if self.count == self.g.len() {
                    return None;
                } else {
                    self.depth += 1;
                    self.stack.push((self.start, self.depth))
                }
            }
        }
    }
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn iddfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = &'a Vertex<T>> {
        if let Some(&i) = self.v_map.get(start) {
            Iddfs::new(self, i)
        } else {
            panic!("Start vertex not in this graph");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_unweighted_edges, from_weighted_edges, make_vertices};

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
            dbg!(v);
        }

        println!("{:->20}", "");
        for v in g1.dfs_last(&a) {
            dbg!(v);
        }

        println!("{:->20}", "");
        for v in g1.iddfs(&a) {
            dbg!(v);
        }
        println!("{:->20}", "");
    }
}
