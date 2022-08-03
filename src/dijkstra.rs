use crate::{Graph, Weight, WeightedEdge};

//use std::collections::HashMap;
use utils::Heap;

/// the difference between dijstra and prim's algorithm:
/// 1. dijstra need to specify a start vertex, while prim needn't
/// 2. the weight push to the heap: d + w vs w
/// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
/// O((E+V)logV) // logV ~ logE
pub fn dijkstra<W, E, G>(graph: &G, start: usize) -> DijkstraIter<W, E, G>
//impl Iterator<Item = (W, usize, usize)> + '_ // TODO: why this did not work
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    DijkstraIter::new(graph, start)
}

pub struct DijkstraIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    graph: &'a G, //Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
}

impl<'a, W, E, G> DijkstraIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    pub fn new(graph: &'a G, start: usize) -> Self {
        let mut heap = Heap::new();
        for e in graph.iter_e_from(start) {
            heap.push((e.weight(), e.to(), start));
        }
        let mut used = vec![false; graph.len()];
        used[start] = true;
        Self { graph, used, heap }
    }
}

impl<'a, W, E, G> Iterator for DijkstraIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((d, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for e in self.graph.iter_e_from(u) {
                    self.heap.push((d + e.weight(), e.to(), u));
                }
                return Some((d, u, v));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_dijkstra() {
        let (g, s_lst) = MakeGraph::mst(true);

        let u = 7;
        println!("All distances from: {}", s_lst[u]); // h
        for (w, u, v) in g.dijkstra(u) {
            println!(
                "to: {}, directly from: {}, distance: {}",
                s_lst[u], s_lst[v], w
            );
        }
    }
}
