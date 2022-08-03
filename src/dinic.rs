use crate::{Graph, Vertex, Weight, EGraph, WeightedEdge, FlowEdge};
use std::collections::VecDeque;

// #[derive(Debug)]
// struct Edge<W: Weight> {
//     from: usize,
//     to: usize,
//     cap: W,
//     flow: W,
// }

// impl<W: Weight> Edge<W> {
//     fn new(from: usize, to: usize, cap: W, flow: W) -> Self {
//         Self {
//             from,
//             to,
//             cap,
//             flow,
//         }
//     }
// }

// struct DinicDfs<W: Weight> {
//     edges: Vec<Edge<W>>,
//     conns: Vec<Vec<usize>>,
//     dist: Vec<i32>,
//     s: usize,
//     t: usize,
// }

// impl<W: Weight> DinicDfs<W> {
//     fn new(edges: Vec<Edge<W>>, conns: Vec<Vec<usize>>, s: usize, t: usize) -> Self {
//         let dist = vec![-1; conns.len()];
//         Self {
//             edges,
//             conns,
//             dist,
//             s,
//             t,
//         }
//         // since we should recalc dist every loop, no need to calc here
//         // obj.calc_dist();
//         // obj
//     }


impl<W: Weight> EGraph<W, FlowEdge<W>> {

    /// use bfs to calc dist
    fn calc_dist(&self, start: usize) -> Vec<i32> {
	let dist = vec![-1; self.len()];

	dist[start] = 0;
	let mut queue = VecDeque::new();
	queue.push_back(start);
	let zero: W = Default::default();
	while let Some(u) = queue.pop_front() {
            for e in self.iter_e_around(u) { //self.conns[u].iter() {
		if e.from == u {
                    let v = e.to;
                    if dist[v] == -1 && e.cap > e.flow {
			queue.push_back(v);
			dist[v] += 1;
                    }
		} else {
                    let v = e.from;
                    if dist[v] == -1 && e.flow > zero {
			queue.push_back(v);
			dist[v] += 1;
                    }
		}
            }
	}
	dist
    }

    /// since no inf can be used, we put the first loop apart
    /// and it's not bad, since as to first loop, no backward will happen
    fn dinic_augment(&mut self, start: usize, dist: &Vec<i32>) -> W {
	let mut flow = Default::default();
	for e in self.iter_e_from(start) { //self.conns[self.s].clone() {
            if dist[e.to] == dist[start] + 1 {
		let nxt = e.to;
		let thres = e.cap - e.flow;
		let f_bak = self.dinic_dfs(nxt, thres);
		e.flow += f_bak; // TODO: only modifed the copyed e
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

// impl<T, W: Weight> Graph<T, W> {
//     /// O(V2E) the proof is also hard
//     /// bfs + dfs, multi-paths augment
//     /// this implement only return the number of max flow (without any path)
//     pub fn dinic(&self, start: &Vertex<T>, target: &Vertex<T>) -> W {
//         let s = self.get_index_of(start).expect("s not in this graph");
//         let t = self.get_index_of(target).expect("t not in this graph");

//         let n = self.len();
//         let mut edges = vec![];
//         let mut conns = vec![vec![]; n]; // vertex's all connected edges (both in and out degrees)
//         for (i, (u, v, w)) in self.iter_edges().into_iter().enumerate() {
//             edges.push(Edge::new(u, v, w, Default::default()));
//             conns[u].push(i);
//             conns[v].push(i);
//         }

//         let mut obj = DinicDfs::new(edges, conns, s, t);
//         let mut flow = Default::default();
//         loop {
//             obj.calc_dist();
//             let flow_add = obj.dinic_augment();
//             if flow_add.is_zero() {
//                 break;
//             }
//             flow += flow_add;
//         }
//         flow
//     }
// }

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
        let (g, s_lst) = MakeGraph::mf();
        let (s, t) = (0, 5);
        dbg!(g.dinic(s, t));
    }
}
