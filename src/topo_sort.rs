use crate::{Graph, Weight};

impl<T, W: Weight> Graph<T, W> {
    /// return partial topological order
    /// i.e. res.len() maybe not equal to v_lst.len()
    /// NOTE: this method used the specail structure of graph
    /// to calculate outdegree
    /// which will be much faster than only used core method
    pub fn topo_sort(&self) -> Vec<usize> {
        let n = self.len();
        let mut count = Vec::with_capacity(n);
        let mut stack = vec![];
        for (i, dct) in self.e_lst.iter().enumerate() {
            // here
            count.push(dct.len());
            if dct.is_empty() {
                stack.push(i);
            }
        }

        let rev_e_lst = self.make_rev_e_lst();
        let mut res: Vec<usize> = Vec::with_capacity(n);
        while let Some(u) = stack.pop() {
            res.push(u);
            for &v in rev_e_lst[u].keys() {
                count[v] -= 1;
                if count[v] == 0 {
                    stack.push(v);
                }
            }
        }
        res
    }
}

impl<T, W: Weight> Graph<T, W> {
    /// It's a special kind of DFS, which yield vertex after all it's out degrees been visited
    /// why in this module: it will only be used in topological sort (and scc)
    /// return: since we always need the entire order, Vec is better than Iterator
    /// cycle: this implement can processs graph which are not DAG (without dead loop)
    /// this implement only promise: **at least one vertex** in scc occur after all scc's out degrees
    pub fn topo_sort_dfs(&self) -> Vec<usize> {
        let n = self.len();
        let mut stack = vec![0];
        let mut visited = vec![0; n];
        let mut count = 0;
        let mut index = 0;
        let mut res = Vec::with_capacity(n);

        loop {
            if let Some(u) = stack.pop() {
                if visited[u] < 2 {
                    visited[u] += 1;
                    if visited[u] == 2 {
                        count += 1;
                        res.push(u);
                    } else {
                        stack.push(u);
                        for &v in self.e_lst[u].keys() {
                            if visited[v] == 0 {
                                stack.push(v);
                            }
                        }
                    }
                }
            } else if count == n {
                return res;
            } else {
                while visited[index] != 0 {
                    index += 1;
                }
                stack.push(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vertex;
    use crate::{add_unweighted_edges, add_vertices, add_weighted_edges};

    #[test]
    fn test_topo_rc() {
        let mut g2: Graph<(), _> = Graph::new();
        add_vertices!(g2 # a, b, c, d, e, f, g, h, i);
        add_weighted_edges!(g2 #
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 10);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7));
        for v in g2.topo_sort() {
            dbg!(&g2[v]);
        }
    }

    #[test]
    fn test_topo_dfs() {
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
        for v in g1.topo_sort_dfs() {
            dbg!(&g1[v]);
        }
    }
}
