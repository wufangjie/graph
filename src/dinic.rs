/// iter_e_around()

use crate::{EGraph, FlowEdge, Graph, Weight};
use std::collections::VecDeque;

pub fn dinic<W: Weight>(graph: &mut EGraph<W, FlowEdge<W>>, start: usize, target: usize) -> W {
    let mut obj = DinicDfs::new(graph.len(), start, target);
    let mut flow = Default::default();
    loop {
        obj.calc_dist(graph);
        let flow_add = obj.dinic_augment(graph);
        if flow_add.is_zero() {
            break;
        }
        flow += flow_add;
    }
    flow
}

struct DinicDfs {
    //graph: &'a mut EGraph<W, FlowEdge<W>>, //edges: Vec<Edge<W>>,
    dist: Vec<i32>,
    start: usize,
    target: usize,
}

impl DinicDfs {
    fn new(n: usize, start: usize, target: usize) -> Self {
        //graph: &'a mut EGraph<W, FlowEdge<W>>,
        let dist = vec![-1; n];
        Self {
            //graph,
            dist,
            start,
            target,
        }
        // since we should recalc dist every loop, no need to calc here
        // obj.calc_dist();
        // obj
    }

    /// use bfs to calc dist
    fn calc_dist<W: Weight>(&mut self, graph: &EGraph<W, FlowEdge<W>>) {
        for d in self.dist.iter_mut() {
            *d = -1;
        }
        self.dist[self.start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        let zero: W = Default::default();
        while let Some(u) = queue.pop_front() {
            for e in graph.iter_e_around(u) {
                if e.from == u {
                    let v = e.to;
                    if self.dist[v] == -1 && e.cap > e.flow {
                        queue.push_back(v);
                        self.dist[v] = self.dist[u] + 1;
                    }
                } else {
                    let v = e.from;
                    if self.dist[v] == -1 && e.flow > zero {
                        queue.push_back(v);
                        self.dist[v] = self.dist[u] + 1;
                    }
                }
            }
        }
    }

    /// since no inf can be used, we put the first loop apart
    /// and it's not bad, since as to first loop, no backward will happen
    fn dinic_augment<W: Weight>(&self, graph: &mut EGraph<W, FlowEdge<W>>) -> W {
        let mut flow = Default::default();

        for i in graph.v_lst[self.start].clone() {
            let e = &graph.e_lst[i];
            if self.dist[e.to] == self.dist[self.start] + 1 {
                let nxt = e.to;
                let thres = e.cap - e.flow;
                let f_bak = self.dinic_dfs(nxt, thres, graph);
                graph.e_lst[i].flow += f_bak;
                flow += f_bak;
            }
        }
        flow
    }

    /// use dfs to find mutli augmenting paths
    fn dinic_dfs<W: Weight>(
        &self,
        cur: usize,
        mut f_max: W,
        graph: &mut EGraph<W, FlowEdge<W>>,
    ) -> W {
        if cur == self.target || f_max.is_zero() {
            return f_max;
        }
        let mut flow: W = Default::default();

        for i in graph.v_lst[cur].clone() {
            let e = &graph.e_lst[i];
            if e.from == cur {
                if self.dist[e.to] == self.dist[cur] + 1 {
                    let nxt = e.to;
                    let thres = min(e.cap - e.flow, f_max);
                    let f_bak = self.dinic_dfs(nxt, thres, graph);
                    graph.e_lst[i].flow += f_bak;
                    flow += f_bak;
                    f_max -= f_bak;
                    if f_max.is_zero() {
                        break;
                    }
                }
            } else if self.dist[e.from] == self.dist[cur] + 1 {
                let nxt = e.from;
                let thres = min(e.flow, f_max);
                let f_bak = self.dinic_dfs(nxt, thres, graph);
                graph.e_lst[i].flow -= f_bak;
                flow += f_bak;
                f_max -= f_bak;
                if f_max.is_zero() {
                    break;
                }
            }
        }
        flow
    }
}

fn min<W: Weight>(x: W, y: W) -> W {
    if x > y {
        y
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_dinic() {
        let (mut g, s_lst) = MakeGraph::mf();
        let (s, t) = (0, 5);
        dbg!(dinic(&mut g, s, t));
    }
}
