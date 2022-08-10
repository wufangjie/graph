// pub mod vertex;
// pub use vertex::Vertex;

use std::collections::{HashMap, HashSet};

pub mod weight;
pub use weight::{NoWeight, Weight};

pub mod graph;
pub use crate::graph::{Graph, VGraph}; // ambiguously?

pub mod edge;
pub use edge::{Edge, FlowEdge, CostFlowEdge}; // two traits

pub mod testing_graph;
pub use testing_graph::MakeGraph;

// following modules mainly used iter_v_from(u)

pub mod bfs;

pub mod dfs;

pub mod topo_sort;

pub mod scc;

// following modules mainly used iter_e_from(u)

pub mod kruskal; // no reverse will be faster

pub mod prim; // need reverse

pub mod dijkstra; // shortest path all need reverse edges
pub use dijkstra::DijkstraIter;

pub mod a_star;

pub mod bellman_ford;

pub mod spfa;

pub mod johnson;

pub mod floyd_warshall;

pub mod bipartite_match; // also used is_empty_from(u)

pub mod vertex_disjoint_path;

pub mod edge_disjoint_path;

pub mod edmonds_karp;

pub mod dinic_new;
// pub mod dinic;

pub mod mcmf;

impl<W: Weight> VGraph<W> {
    pub fn bfs(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
	bfs::bfs(self, start)
    }

    pub fn dfs(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
	dfs::dfs(self, start)
    }

    pub fn iddfs(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
	dfs::iddfs(self, start)
    }

    pub fn topo_sort_rc(&self) -> Vec<usize> {
	topo_sort::topo_sort_rc(self)
    }

    pub fn topo_sort_dfs(&self) -> Vec<usize> {
	topo_sort::topo_sort_dfs(self)
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
	scc::scc(self)
    }

    pub fn kruskal(&self) -> Vec<(W, usize, usize)> {
        kruskal::kruskal(self)
    }

    pub fn prim(&self) -> Vec<(W, usize, usize)> {
        prim::prim(self)
    }

    pub fn dijkstra(&self, start: usize) -> impl Iterator<Item = (W, usize, usize)> + '_ {
        dijkstra::dijkstra(self, start)
    }

    pub fn a_star<F: Fn(usize) -> W + 'static>(&self, start: usize, func: F) -> impl Iterator<Item = (W, usize, usize)> + '_ {
        a_star::a_star(self, start, func)
    }

    pub fn bellman_ford(&self, start: usize) -> (bool, Vec<Option<W>>, Vec<usize>) {
        bellman_ford::bellman_ford(self, start)
    }

    pub fn spfa(&self, start: usize) -> (bool, Vec<Option<W>>, Vec<usize>) {
	spfa::spfa(self, start)
    }

    pub fn johnson(&self) -> Vec<(Vec<Option<W>>, Vec<usize>)> {
	johnson::johnson(self)
    }

    pub fn floyd_warshall(&self) -> Vec<Vec<Option<W>>> {
	floyd_warshall::floyd_warshall(self)
    }

    pub fn bipartite_match(&self) -> HashMap<usize, usize> {
	bipartite_match::bipartite_match(self)
    }

    pub fn vertex_disjoint_path(&self, start: usize, target: usize) -> (HashMap<usize, usize>, Vec<usize>) {
	vertex_disjoint_path::vertex_disjoint_path(self, start, target)
    }

    pub fn edge_disjoint_path(&self, start: usize, target: usize) -> HashMap<usize, HashSet<usize>> {
	edge_disjoint_path::edge_disjoint_path(self, start, target)
    }

    pub fn edmonds_karp(&self, start: usize, target: usize) -> HashMap<usize, HashMap<usize, W>> {
	edmonds_karp::edmonds_karp(self, start, target)
    }

}


// impl<W, E> EGraph<W, E>
// where
//     W: Weight,
//     E: WeightedEdge<W>
// {
//     pub fn kruskal(&self) -> Vec<(W, usize, usize)> {
//         kruskal::kruskal(self)
//     }

//     // TODO: add other impls
//     pub fn edmonds_karp(&self, start: usize, target: usize) -> HashMap<usize, HashMap<usize, W>> {
// 	edmonds_karp::edmonds_karp(self, start, target)
//     }
// }
