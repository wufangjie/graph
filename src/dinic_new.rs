/// iter_e_from(), iter_v_to()
use crate::{Graph, Weight};
use std::collections::{HashMap, VecDeque};

pub fn dinic<G: Graph>(
    graph: &G,
    start: usize,
    target: usize,
) -> HashMap<usize, HashMap<usize, G::Weight>> {
    let mut obj = DinicDfs::new(graph, start, target);
    //let mut flow = Default::default();
    let mut flowing = HashMap::new();
    loop {
        obj.calc_dist(&mut flowing);
        let flow_add = obj.dinic_augment(&mut flowing);
        if flow_add.is_zero() {
            break;
        }
        //flow += flow_add;
    }
    //flow
    flowing
}

struct DinicDfs<'a, G: Graph> {
    graph: &'a G,
    dist: Vec<i32>,
    start: usize,
    target: usize,
}

impl<'a, G: Graph> DinicDfs<'a, G> {
    fn new(graph: &'a G, start: usize, target: usize) -> Self {
        let dist = vec![-1; graph.len()];
        Self {
            graph,
            dist,
            start,
            target,
        }
        // NOTE: since we should recalc dist every loop, no need to calc here
    }

    /// use bfs to calc dist
    fn calc_dist(&mut self, flowing: &mut HashMap<usize, HashMap<usize, G::Weight>>) {
        for d in self.dist.iter_mut() {
            *d = -1;
        }
        self.dist[self.start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        let zero = Default::default();
        while let Some(u) = queue.pop_front() {
            for (v, w) in self.graph.iter_e_from(u) {
                if self.dist[v] == -1 && w > get_flow_in(flowing, v, u) {
                    queue.push_back(v);
                    self.dist[v] = self.dist[u] + 1;
                }
            }
            for v in self.graph.iter_v_to(u) {
                if self.dist[v] == -1 && get_flow_in(flowing, u, v) > zero {
                    queue.push_back(v);
                    self.dist[v] = self.dist[u] + 1;
                }
            }
        }
    }

    /// since no inf can be used, we put the first loop apart
    /// and it's not bad, since as to first loop, no backward will happen
    fn dinic_augment(&self, flowing: &mut HashMap<usize, HashMap<usize, G::Weight>>) -> G::Weight {
        let mut flow = Default::default();

        for (v, w) in self.graph.iter_e_from(self.start) {
            if self.dist[v] == self.dist[self.start] + 1 {
                let thres = w - get_flow_in(flowing, v, self.start);
                let f_bak = self.dinic_dfs(v, thres, flowing);
                add_flow_in(flowing, v, self.start, f_bak);
                flow += f_bak;
            }
        }
        flow
    }

    /// use dfs to find mutli augmenting paths
    fn dinic_dfs(
        &self,
        cur: usize,
        mut f_max: G::Weight,
        flowing: &mut HashMap<usize, HashMap<usize, G::Weight>>,
    ) -> G::Weight {
        if cur == self.target || f_max.is_zero() {
            return f_max;
        }
        let mut flow = Default::default();

        for (v, w) in self.graph.iter_e_from(cur) {
            if self.dist[v] == self.dist[cur] + 1 {
                let thres = f_max.min(w - get_flow_in(flowing, v, cur));
                let f_bak = self.dinic_dfs(v, thres, flowing);
                add_flow_in(flowing, v, cur, f_bak);
                f_max -= f_bak;
                flow += f_bak;
                if f_max.is_zero() {
                    return flow;
                }
            }
        }

        for v in self.graph.iter_v_to(cur) {
            if self.dist[v] == self.dist[cur] + 1 {
                let thres = f_max.min(get_flow_in(flowing, cur, v));
                let f_bak = self.dinic_dfs(v, thres, flowing);
                add_flow_in(flowing, cur, v, f_bak);
                f_max -= f_bak;
                flow += f_bak;
                if f_max.is_zero() {
                    break;
                }
            }
        }
        flow
    }
}

/// return value in flowing[u][v]
fn get_flow_in<W: Weight>(flowing: &HashMap<usize, HashMap<usize, W>>, u: usize, v: usize) -> W {
    if let Some(dct) = flowing.get(&u) {
        if let Some(w) = dct.get(&v) {
            return *w;
        }
    }
    Default::default()
}

/// add flow in flowing[u][v]
fn add_flow_in<W: Weight>(
    flowing: &mut HashMap<usize, HashMap<usize, W>>,
    u: usize,
    v: usize,
    delta: W,
) {
    if let Some(dct) = flowing.get_mut(&u) {
        if let Some(w) = dct.get_mut(&v) {
            *w += delta;
        } else {
            dct.insert(v, delta);
        }
    } else {
        flowing.insert(u, HashMap::from([(v, delta)]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_dinic() {
        let (g, s_lst) = MakeGraph::mf();
        let (s, t) = (0, 5);

        let flowing = dinic(&g, s, t);
        for (v, dct) in flowing.into_iter() {
            print!("to {}:\tfrom: {{", s_lst[v]);
            let process_tail = !dct.is_empty();
            for (u, w) in dct.into_iter() {
                print!(" {}: {},", s_lst[u], w);
            }
            if process_tail {
                print!("\x08 ");
            }
            println!("}}");
        }
    }
}
