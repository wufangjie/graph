use crate::Graph;

pub fn dfs<G: Graph>(graph: &G, start: usize) -> impl Iterator<Item = usize> + '_ {
    DfsIter::new(graph, start)
}

pub fn iddfs<G: Graph>(graph: &G, start: usize) -> impl Iterator<Item = usize> + '_ {
    IddfsIter::new(graph, start)
}

/// dfs helper
/// yield vertex as soon as dfs reach it
struct DfsIter<'a, G: Graph> {
    visited: Vec<bool>,
    stack: Vec<usize>,
    graph: &'a G,
}

impl<'a, G: Graph> DfsIter<'a, G> {
    fn new(graph: &'a G, start: usize) -> Self {
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

/// iddfs helper
/// Iterative Deepening Depth-First Search
/// There is really only one situation where IDDFS would be preferable over BFS:
/// when searching a huge acyclic graph
/// (saving a significant amount of memory, with little or no asymptotic slowdown)
struct IddfsIter<'a, G: Graph> {
    visited: Vec<bool>,
    stack: Vec<(usize, usize)>,
    graph: &'a G,
    start: usize,
    count: usize,
    depth: usize,
}

impl<'a, G: Graph> IddfsIter<'a, G> {
    /// maybe we should provide a parameter to set init depth?
    fn new(graph: &'a G, start: usize) -> Self {
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

impl<'a, G: Graph> Iterator for IddfsIter<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: depth's upper bound: the number of vertices
        while self.depth < self.graph.len() {
            if let Some((u, d)) = self.stack.pop() {
                if d > 0 {
                    for v in self.graph.iter_v_from(u) {
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
    use crate::MakeGraph;

    #[test]
    fn test_dfs() {
        let (g, s_lst) = MakeGraph::scc();
        for v in g.dfs(0) {
            dbg!(s_lst[v]);
        }

        println!("{:->20}", "");

        for v in g.iddfs(0) {
            dbg!(s_lst[v]);
        }
    }
}
