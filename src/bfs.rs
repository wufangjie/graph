use crate::{Graph, Vertex};
use std::collections::VecDeque;
use std::hash::Hash;

struct BfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    visited: Vec<bool>,
    queue: VecDeque<usize>,
    g: &'a Graph<T, W>,
}

impl<'a, T, W> BfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    fn new(g: &'a Graph<T, W>, start: usize) -> Self {
        let n = g.v_lst.len();
        let mut visited = vec![false; n];
        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        Self { visited, queue, g }
    }
}

impl<'a, T, W> Iterator for BfsIter<'a, T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(u) = self.queue.pop_front() {
            for &v in self.g.e_lst[u].keys() {
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

impl<T, W> Graph<T, W>
where
    T: Eq + Hash + Clone,
    W: Clone + Copy + Default,
{
    pub fn bfs<'a>(&'a self, start: &Vertex<T>) -> impl Iterator<Item = usize> + 'a {
        if let Some(&i) = self.v_map.get(start) {
            BfsIter::new(self, i)
        } else {
            panic!("Vertex not in this graph");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{from_unweighted_edges, make_vertices};

    #[test]
    fn test_bfs() {
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

        for v in g1.bfs(&a) {
            dbg!(&g1[v]);
        }
    }
}
