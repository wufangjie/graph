use crate::{Graph, Weight, WeightedEdge};

use utils::Heap;

/// the difference between dijstra and prim's algorithm:
/// 1. dijstra need to specify a start vertex, while prim needn't
/// 2. the weight push to the heap: d + w vs w
/// 3. [NOT Algorithm] dijkstra works on directed graph, while prim on undirected graph
pub fn a_star<W, E, G, F>(graph: &G, start: usize, func: F) -> AStarIter<W, E, G, F>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
    F: Fn(usize) -> W,
{
    AStarIter::new(graph, start, func)
}

pub struct AStarIter<'a, W, E, G, F>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
    F: Fn(usize) -> W,
{
    graph: &'a G, //Vec<HashMap<usize, W>>,
    used: Vec<bool>,
    heap: Heap<(W, usize, usize)>,
    func: F,
}

impl<'a, W, E, G, F> AStarIter<'a, W, E, G, F>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
    F: Fn(usize) -> W,
{
    fn new(graph: &'a G, start: usize, func: F) -> Self {
        let mut heap = Heap::new();
        for e in graph.iter_e_from(start) {
            heap.push((e.weight() + func(e.to()), e.to(), start));
        }
        let mut used = vec![false; graph.len()];
        used[start] = true;
        Self {
            graph,
            used,
            heap,
            func,
        }
    }
}

impl<'a, W, E, G, F> Iterator for AStarIter<'a, W, E, G, F>
where
    W: Weight,
    E: WeightedEdge<W>,
    G: Graph<Edge = E>,
    F: Fn(usize) -> W,
{
    type Item = (W, usize, usize);

    fn next(&mut self) -> Option<(W, usize, usize)> {
        while let Some((d, u, v)) = self.heap.pop() {
            if !self.used[u] {
                self.used[u] = true;
                let hu = (self.func)(u);
                for e in self.graph.iter_e_from(u) {
                    let w = e.weight();
                    let v = e.to();
                    self.heap.push((d + w - hu + (self.func)(v), v, u));
                }
                return Some((d - hu, u, v));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_a_star() {
        let (g, s_lst, xy) = MakeGraph::spa();

        println!("All distances from: {}", s_lst[0]); // `s`
        let calc_dist_to_t = move |u| {
            let (x0, y0): (f64, f64) = xy[6]; // `t`'s xy
            let (x1, y1): (f64, f64) = xy[u];
            ((x1 - x0).powi(2) + (y1 - y0).powi(2)).powf(0.5)
        };

        for (w, u, v) in g.a_star(0, calc_dist_to_t) {
            println!(
                "to: {}, directly from: {}, distance: {:.1}",
                s_lst[u], s_lst[v], w
            );
        }
    }
}
