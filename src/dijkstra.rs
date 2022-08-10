use crate::Graph;
use utils::Heap;

/// the difference between dijstra and prim's algorithm:
/// 1. dijstra need to specify a start vertex, while prim needn't
/// 2. the weight push to the heap: d + w vs w
/// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
/// O((E+V)logV) // logV ~ logE
pub fn dijkstra<G: Graph>(
    graph: &G,
    start: usize,
) -> impl Iterator<Item = (G::Weight, usize, usize)> + '_ {
    //DijkstraIter<G>
    DijkstraIter::new(graph, start)
}

pub struct DijkstraIter<'a, G: Graph> {
    graph: &'a G,
    used: Vec<bool>,
    heap: Heap<(G::Weight, usize, usize)>,
}

impl<'a, G: Graph> DijkstraIter<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let mut heap = Heap::new();
        for (v, w) in graph.iter_e_from(start) {
            heap.push((w, v, start));
        }
        let mut used = vec![false; graph.len()];
        used[start] = true;
        Self { graph, used, heap }
    }
}

impl<'a, G: Graph> Iterator for DijkstraIter<'a, G> {
    type Item = (G::Weight, usize, usize);

    fn next(&mut self) -> Option<(G::Weight, usize, usize)> {
        while let Some((d, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for (v, w) in self.graph.iter_e_from(u) {
                    self.heap.push((d + w, v, u));
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
