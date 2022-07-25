use crate::{Graph, Vertex, Weight};
use std::collections::VecDeque;

#[derive(Debug)]
struct Edge<W: Weight> {
    from: usize,
    to: usize,
    cap: W,
    flow: W,
}

impl<W: Weight> Edge<W> {
    fn new(from: usize, to: usize, cap: W, flow: W) -> Self {
        Self {
            from,
            to,
            cap,
            flow,
        }
    }
}

struct DinicDfs<W: Weight> {
    edges: Vec<Edge<W>>,
    conns: Vec<Vec<usize>>,
    dist: Vec<i32>,
    s: usize,
    t: usize,
}

impl<W: Weight> DinicDfs<W> {
    fn new(edges: Vec<Edge<W>>, conns: Vec<Vec<usize>>, s: usize, t: usize) -> Self {
        let dist = vec![-1; conns.len()];
        Self {
            edges,
            conns,
            dist,
            s,
            t,
        }
        // since we should recalc dist every loop, no need to calc here
        // obj.calc_dist();
        // obj
    }

    /// use bfs to calc dist
    fn calc_dist(&mut self) {
        for d in self.dist.iter_mut() {
            *d = -1;
        }
        self.dist[self.s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(self.s);
        let zero: W = Default::default();
        while let Some(u) = queue.pop_front() {
            for &i in self.conns[u].iter() {
                if self.edges[i].from == u {
                    let v = self.edges[i].to;
                    if self.dist[v] == -1 && self.edges[i].cap > self.edges[i].flow {
                        queue.push_back(v);
                        self.dist[v] = self.dist[u] + 1;
                    }
                } else {
                    let v = self.edges[i].from;
                    if self.dist[v] == -1 && self.edges[i].flow > zero {
                        queue.push_back(v);
                        self.dist[v] = self.dist[u] + 1;
                    }
                }
            }
        }
    }

    /// since no inf can be used, we put the first loop apart
    /// and it's not bad, since as to first loop, no backward will happen
    fn dinic_augment(&mut self) -> W {
        let mut flow = Default::default();
        for i in self.conns[self.s].clone() {
            let e = &self.edges[i];
            if self.dist[e.to] == self.dist[self.s] + 1 {
                let nxt = e.to;
                let thres = e.cap - e.flow;
                let f_bak = self.dinic_dfs(nxt, thres);
                self.edges[i].flow += f_bak;
                flow += f_bak;
            }
        }
        flow
    }

    /// use dfs to find mutli augmenting paths
    fn dinic_dfs(&mut self, cur: usize, mut f_max: W) -> W {
        if cur == self.t || f_max.is_zero() {
            return f_max;
        }
        let mut flow: W = Default::default();

        for i in self.conns[cur].clone() {
            let e = &self.edges[i];
            if e.from == cur {
                if self.dist[e.to] == self.dist[cur] + 1 {
                    let nxt = e.to;
                    let thres = min(e.cap - e.flow, f_max);
                    let f_bak = self.dinic_dfs(nxt, thres);
                    self.edges[i].flow += f_bak;
                    flow += f_bak;
                    f_max -= f_bak;
                    if f_max.is_zero() {
                        break;
                    }
                }
            } else if self.dist[e.from] == self.dist[cur] + 1 {
                let nxt = e.to;
                let thres = min(e.flow, f_max);
                let f_bak = self.dinic_dfs(nxt, thres);
                self.edges[i].flow -= f_bak;
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

impl<T, W: Weight> Graph<T, W> {
    /// O(V2E) the proof is also hard
    /// bfs + dfs, multi-paths augment
    /// this implement only return the number of max flow (without any path)
    pub fn dinic(&self, start: &Vertex<T>, target: &Vertex<T>) -> W {
        let s = self.get_index_of(start).expect("s not in this graph");
        let t = self.get_index_of(target).expect("t not in this graph");

        let n = self.len();
        let mut edges = vec![];
        let mut conns = vec![vec![]; n]; // vertex's all connected edges (both in and out degrees)
        for (i, (u, v, w)) in self.iter_edges().into_iter().enumerate() {
            edges.push(Edge::new(u, v, w, Default::default()));
            conns[u].push(i);
            conns[v].push(i);
        }

        let mut obj = DinicDfs::new(edges, conns, s, t);
        let mut flow = Default::default();
        loop {
            obj.calc_dist();
            let flow_add = obj.dinic_augment();
            if flow_add.is_zero() {
                break;
            }
            flow += flow_add;
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
    use crate::{add_vertices, add_weighted_edges};

    #[test]
    fn test_dinic() {
        let mut g8: Graph<(), i32> = Graph::new();
        add_vertices!(g8 # s, v1, v2, v3, v4, t);
        add_weighted_edges!(g8 #
			    s: (v1, 16), (v2, 13);
			    v1: (v3, 12);
			    v2: (v1, 4), (v4, 14);
			    v3: (v2, 9), (t, 20);
			    v4: (v3, 7), (t, 4));

        dbg!(g8.dinic(&s, &t));
    }
}
