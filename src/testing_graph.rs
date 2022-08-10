/// this module provide some testing graphs for doing experiment
use crate::{make_symbol_lst, make_vertices, make_vertices_rec};
use crate::{NoWeight, VGraph};//, EGraph, FlowEdge, CostFlowEdge};
use std::collections::HashMap;

pub struct MakeGraph;

impl MakeGraph {
    /// Strongly Connected Components
    /// actually it's good for testing bfs/dfs/topological_sort
    pub fn scc() -> (VGraph<NoWeight>, Vec<&'static str>) {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let s_lst = make_symbol_lst!(a, b, c, d, e, f, g, h, i);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v) in [
            (a, b),
            (a, c),
            (b, d),
            (b, e),
            (b, i),
            (c, d),
            (d, a),
            (d, h),
            (e, f),
            (f, g),
            (g, e),
            (g, h),
            (h, i),
            (i, h),
        ]
        .into_iter()
        {
            lst[u].insert(v, NoWeight);
        }
        (VGraph::new(lst), s_lst)
    }

    /// Minimum Spanning Tree
    /// for prim/kruskal/bellman_ford/dijkstra
    pub fn mst(add_rev: bool) -> (VGraph<i32>, Vec<&'static str>) {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let s_lst = make_symbol_lst!(a, b, c, d, e, f, g, h, i);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v, w) in [
            (a, b, 4),
            (a, h, 8),
            (b, c, 8),
            (b, h, 11),
            (c, d, 7),
            (c, f, 4),
            (c, i, 2),
            (d, e, 9),
            (d, f, 14),
            (e, f, 10),
            (f, g, 2),
            (g, h, 1),
            (g, i, 6),
            (h, i, 7),
        ]
        .into_iter()
        {
            lst[u].insert(v, w);
            if add_rev {
                lst[v].insert(u, w);
            }
        }
        (VGraph::new(lst), s_lst)
    }

    /// Graph carry additional data (for example: x, y coordinate)
    pub fn spa() -> (VGraph<f64>, Vec<&'static str>, Vec<(f64, f64)>) {
        make_vertices!(s, a, b, c, d, e, t);
        let s_lst = make_symbol_lst!(s, a, b, c, d, e, t);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v, w) in [
            (s, a, 3.0),
            (s, d, 2.0),
            (a, b, 2.0),
            (b, c, 3.0),
            (c, t, 3.0),
            (d, e, 4.0),
            (e, t, 4.5),
        ]
        .into_iter()
        {
            lst[u].insert(v, w);
            lst[v].insert(u, w); // always need backward edges
        }

        let dx = 3f64.powf(0.5);
        let xy = vec![
            (1.0, 4.0),
            (1.0 + dx, 5.0),
            (1.0 + dx, 3.0),
            (1.0 + 2.0 * dx, 5.0),
            (1.0, 2.0),
            (1.0 + dx, 1.0),
            (1.0 + 3.0 * dx, 3.0),
        ];

        (VGraph::new(lst), s_lst, xy)
    }

    /// Shortest Path with Negative weight edge
    /// for johnson/floyd_warshall
    pub fn spn() -> (VGraph<i32>, Vec<&'static str>) {
        make_vertices!(v1, v2, v3, v4, v5);
        let s_lst = make_symbol_lst!(v1, v2, v3, v4, v5);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v, w) in [
            (v1, v2, 3),
            (v1, v3, 8),
            (v1, v5, -4),
            (v2, v4, 1),
            (v2, v5, 7),
            (v3, v2, 4),
            (v4, v1, 2),
            (v4, v3, -5),
            (v5, v4, 6),
        ]
        .into_iter()
        {
            lst[u].insert(v, w);
        }
        (VGraph::new(lst), s_lst)
    }

    /// Maximum Bipartite Matching
    pub fn mbm() -> (VGraph<NoWeight>, Vec<&'static str>) {
        make_vertices!(x1, x2, x3, x4, x5, x6, y1, y2, y3, y4, y5, y6);
        let s_lst = make_symbol_lst!(x1, x2, x3, x4, x5, x6, y1, y2, y3, y4, y5, y6);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v) in [
            (x1, y1),
            (x1, y4),
            (x2, y1),
            (x2, y2),
            (x2, y5),
            (x3, y2),
            (x3, y3),
            (x3, y6),
            (x4, y3),
            (x5, y6),
            (x6, y5),
        ]
        .into_iter()
        {
            lst[u].insert(v, NoWeight);
        }
        (VGraph::new(lst), s_lst)
    }

    /// Disjoint Path
    /// for edge/vertex disjoint path
    pub fn dp() -> (VGraph<NoWeight>, Vec<&'static str>) {
        make_vertices!(s, a, b, c, d, e, t);
        let s_lst = make_symbol_lst!(s, a, b, c, d, e, t);
        let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v) in [
            (s, a),
            (s, c),
            (s, e),
            (a, b),
            (b, t),
            (c, b),
            (c, d),
            (c, t),
            (d, t),
            (e, c),
        ]
        .into_iter()
        {
            lst[u].insert(v, NoWeight);
        }
        (VGraph::new(lst), s_lst)
    }

    pub fn mf() -> (VGraph<i32>, Vec<&'static str>) {
        make_vertices!(s, v1, v2, v3, v4, t);
	let s_lst = make_symbol_lst!(s, v1, v2, v3, v4, t);
	let mut lst = vec![HashMap::new(); s_lst.len()];
        for (u, v, w) in [
	    (s, v1, 16),
            (s, v2, 13),
            (v1, v3, 12),
            (v2, v1, 4),
            (v2, v4, 14),
            (v3, v2, 9),
            (v3, t, 20),
            (v4, v3, 7),
            (v4, t, 4)]
	    .into_iter() {
		lst[u].insert(v, w);
	    }
	(VGraph::new(lst), s_lst)
    }

    #[allow(clippy::type_complexity)]
    pub fn mcmf() -> (VGraph<i32>, Vec<&'static str>, HashMap<(usize, usize), i32>) {
        make_vertices!(s, v1, v2, v3, v4, t);
	let s_lst = make_symbol_lst!(s, v1, v2, v3, v4, t);
	let mut lst = vec![HashMap::new(); s_lst.len()]; // cost graph
        for (u, v, w) in [
	    (s, v1, 3), // change 1 <-> 3 see what happened
            (s, v2, 1),
            (v1, v3, 1),
            (v2, v1, 1),
            (v2, v4, 1),
            (v3, v2, 1),
            (v3, t, 1),
            (v4, v3, 1),
            (v4, t, 1)]
	    .into_iter() {
		lst[u].insert(v, w);
	    }
	let mut cap_dct = HashMap::new();
	for (u, v, w) in [
	    (s, v1, 16),
            (s, v2, 13),
            (v1, v3, 12),
            (v2, v1, 4),
            (v2, v4, 14),
            (v3, v2, 9),
            (v3, t, 20),
            (v4, v3, 7),
            (v4, t, 4)]
	    .into_iter() {
		cap_dct.insert((u, v), w);
	    }
	(VGraph::new(lst), s_lst, cap_dct)
    }
}
