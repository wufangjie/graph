/// Minimum Cost Maximum Flow
use crate::{Graph, Weight};
/// use std::collections::VecDeque;
use std::collections::HashMap;

type Flowing<W> = Vec<HashMap<usize, (W, W)>>;

/// find shortest path on residual cost networks,
/// until no augment path can be found
/// we find shortest path on residual cost networks,
/// using something like augmenting path,
/// we can get the fact: we just find the minimum cost paths on original network

/// proof
/// each step: we have original shortest path a, and next shortest path b,
/// if not greedy we can found c and d, so that c + d < a + b
/// obviously we have c <= a, d <= a, so we need c < b and d < b
/// if one of c or d share no common edge of a, then b is not the shortest path
/// else we can break and remake c + d to a + e,
/// since b is the shortest we have e >= b, so c + d >= a + b
/// NOTE: cost are always positive, but we can have negative cost on residual networks
pub fn mcmf<G: Graph>(
    graph: &G, // cost graph
    cap_dct: &HashMap<(usize, usize), G::Weight>,
    start: usize,
    target: usize,
) -> (bool, Flowing<G::Weight>) {
    let zero = Default::default();

    let mut rgraph = Residual::new(graph, cap_dct);
    loop {
        let (_state, dist, from) = rgraph.spfa(start);
        match dist[target] {
            Some(d) => {
                if d < zero {
                    return (false, rgraph.flowing);
                }
                let mut v = target;
                let mut flow = rgraph.get_residual_forward_flow(from[v], v);
                while v != start {
                    flow = flow.min(rgraph.get_residual_flow(from[v], v));
                    v = from[v];
                }

                let mut v = target;
                while v != start {
                    let u = from[v];
                    if cap_dct.contains_key(&(u, v)) {
                        rgraph.add_forward_flow(u, v, flow);
                    } else {
                        rgraph.add_backward_flow(u, v, flow);
                    }
                    v = u;
                }
            }
            None => return (true, rgraph.flowing),
        }
    }
}

struct Residual<'a, G: Graph> {
    graph: &'a G, // NOTE: need add reverse edge by hand
    flowing: Flowing<G::Weight>,
}

impl<'a, G: Graph> Residual<'a, G> {
    // NOTE: make flowing carefully

    fn new(graph: &'a G, cap_dct: &'a HashMap<(usize, usize), G::Weight>) -> Self {
        let n = graph.len();
        let mut flowing = vec![HashMap::new(); n];
        let zero = Default::default();
        for u in 0..n {
            for v in graph.iter_v_from(u) {
                let w = *cap_dct.get(&(u, v)).unwrap();
                flowing[v].insert(u, (w, zero));
            }
        }
        Self { graph, flowing }
    }

    fn get_residual_forward_flow(&self, u: usize, v: usize) -> G::Weight {
        let (cap, flow) = self.flowing[v].get(&u).unwrap();
        *cap - *flow
    }

    fn get_residual_backward_flow(&self, u: usize, v: usize) -> G::Weight {
        let (_cap, flow) = self.flowing[u].get(&v).unwrap();
        *flow
    }

    fn get_residual_flow(&self, u: usize, v: usize) -> G::Weight {
        if let Some((cap, flow)) = self.flowing[v].get(&u) {
            *cap - *flow
        } else {
            self.get_residual_backward_flow(u, v)
        }
    }

    /// flow(u -> v) < cap(u -> v), it's ok to unwrap
    fn can_add_forward_flow(&self, u: usize, v: usize) -> bool {
        self.get_residual_forward_flow(u, v).is_positive()
    }

    /// flow(u -> v) > 0, it's ok to unwrap
    fn can_add_backward_flow(&self, u: usize, v: usize) -> bool {
        self.get_residual_backward_flow(u, v).is_positive()
    }

    fn add_forward_flow(&mut self, u: usize, v: usize, w: G::Weight) {
        let p = self.flowing[v].get_mut(&u).unwrap();
        *p = (p.0, p.1 + w);
    }

    fn add_backward_flow(&mut self, u: usize, v: usize, w: G::Weight) {
        let p = self.flowing[u].get_mut(&v).unwrap();
        *p = (p.0, p.1 - w);
    }

    fn spfa(&self, start: usize) -> (bool, Vec<Option<G::Weight>>, Vec<usize>) {
        crate::spfa::spfa(self, start)
    }
}

impl<'a, G: Graph> Graph for Residual<'a, G> {
    type Weight = G::Weight;

    fn len(&self) -> usize {
        self.graph.len()
    }

    fn iter_v_from(&self, _u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        unimplemented!();
    }

    fn iter_v_to(&self, _u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        unimplemented!();
    }

    /// iter all the edges from vertex `u`
    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_> {
        // TODO:
        let zero: Self::Weight = Default::default();
        let iter_forward = self
            .graph
            .iter_e_from(u)
            .filter(move |e| self.can_add_forward_flow(u, e.0));
        let iter_backward = self
            .graph
            .iter_e_to(u)
            .filter(move |e| self.can_add_backward_flow(u, e.0))
            .map(move |(v, w)| (v, zero - w));
        Box::new(iter_forward.chain(iter_backward))
    }

    /// iter all the edges to vertex `u`
    fn iter_e_to(&self, _u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_mcmf() {
        let (g, s_lst, cap_dct) = MakeGraph::mcmf();
        let (s, t) = (0, 5);
        let (_state, flowing) = mcmf(&g, &cap_dct, s, t);

        for (v, dct) in flowing.into_iter().enumerate() {
            print!("to {}:\tfrom: {{", s_lst[v]);
            let process_tail = !dct.is_empty();
            for (u, w) in dct.into_iter() {
                print!(" {}: {:?},", s_lst[u], w);
            }
            if process_tail {
                print!("\x08 ");
            }
            println!("}}");
        }
    }
}
