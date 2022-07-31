use crate::{Graph, Weight, WeightedEdge};
use utils::Heap;

/// run prim on directed graph (need to add reverse edges)
/// it is only can be used on the graph,
/// which exist (a -> b: w) then (b -> a: w),
/// otherwise we will get bad result
/// O((E+V)logV)
pub fn prim<W, E, G>(graph: &G) -> Vec<(W, usize, usize)>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    PrimIter::new(graph).collect()
}

struct PrimIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    graph: &'a G, //Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
}

impl<'a, W, E, G> PrimIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    fn new(graph: &'a G) -> Self {
        let start = 0;
        let mut heap = Heap::new();
        for e in graph.iter_e_from(start) {
            heap.push((e.weight(), e.to(), e.from()));
        }
        let mut used = vec![false; graph.len()];
        used[start] = true;
        Self { graph, used, heap }
    }
}

impl<'a, W, E, G> Iterator for PrimIter<'a, W, E, G>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((w, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for e in self.graph.iter_e_from(u) {
                    self.heap.push((e.weight(), e.to(), e.from()));
                }
                return Some((w, u, v)); // NOTE: v, w is ok
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_prim() {
        let g = MakeGraph::mst(true);
        let res = prim(&g);
        assert_eq!(res.iter().map(|(w, _u, _v)| *w).sum::<i32>(), 37);
        dbg!(res);
    }
}
