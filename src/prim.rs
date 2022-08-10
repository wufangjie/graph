use crate::Graph;
use utils::Heap;

/// run prim on directed graph (need to add reverse edges)
/// it is only can be used on the graph,
/// which exist (a -> b: w) then (b -> a: w),
/// otherwise we will get bad result
/// O((E+V)logV)
pub fn prim<G: Graph>(graph: &G) -> Vec<(G::Weight, usize, usize)> {
    PrimIter::new(graph).collect()
}

struct PrimIter<'a, G: Graph> {
    graph: &'a G,
    used: Vec<bool>,
    heap: Heap<(G::Weight, usize, usize)>,
}

impl<'a, G: Graph> PrimIter<'a, G> {
    fn new(graph: &'a G) -> Self {
        let start = 0;
        let mut heap = Heap::new();
        for (v, w) in graph.iter_e_from(start) {
            heap.push((w, v, start));
        }
        let mut used = vec![false; graph.len()];
        used[start] = true;
        Self { graph, used, heap }
    }
}

impl<'a, G: Graph> Iterator for PrimIter<'a, G> {
    type Item = (G::Weight, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((w, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                for (v, w) in self.graph.iter_e_from(u) {
                    self.heap.push((w, v, u));
                }
                return Some((w, u, v)); // NOTE: v, w is ok
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_prim() {
        let (g, s_lst) = MakeGraph::mst(true);
        let res = g.prim();
        assert_eq!(res.iter().map(|(w, _u, _v)| *w).sum::<i32>(), 37);
        for (w, u, v) in res.into_iter() {
            println!("weight: {}, from: {}, to: {}", w, s_lst[u], s_lst[v]);
        }
    }
}
