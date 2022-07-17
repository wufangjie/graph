/// This module impl graph's basic data structures and some useful Macros
/// NOTE: use Graph<T, NoWeight> to present unweighted graph (NoWeight is a zst)
/// Macros:
/// 1. from_edges_nw
/// 2. from_edges_ww
use crate::{Vertex, Weight};
//use std::ops::PartialEq;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Graph<T, W: Weight> {
    pub(crate) v_lst: Vec<Vertex<T>>,
    pub(crate) e_lst: Vec<HashMap<usize, W>>, // edges
}

/// Core methods a graph should implement
/// I tried to implement a graph trait, but failed
/// because trait methods can not return impl Iterator
/// TODO: if this core methods is well designed?
impl<T, W: Weight> Graph<T, W> {
    pub fn len(&self) -> usize {
        self.v_lst.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }

    /// return all vertex: u's outdegree as an iterator
    pub fn iter_vertices_from(&self, u: usize) -> impl Iterator<Item = usize> + '_ {
        self.e_lst[u].keys().cloned()
    }

    /// NOTE: we can not elide '_, and we don't to add 'a by hand
    pub fn iter_edges_from(&self, u: usize) -> impl Iterator<Item = (usize, W)> + '_ {
        self.e_lst[u].iter().map(|(&v, &w)| (v, w))
    }

    /// TODO: make it iterable, (it's easy and elegant if rust can use yield)
    pub fn iter_edges(&self) -> Vec<(usize, usize, W)> {
        let mut res = vec![]; // with_capacity?
        for u in self.iter_vertices() {
            for (v, w) in self.iter_edges_from(u) {
                res.push((u, v, w));
            }
        }
        res
    }
}

impl<T, W: Weight> Default for Graph<T, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, W: Weight> Graph<T, W> {
    pub fn new() -> Self {
        Self {
            v_lst: Default::default(),
            e_lst: Default::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vertex<T>> {
        self.v_lst.iter()
    }

    pub fn get_index_of(&self, vertex: &Vertex<T>) -> Option<usize> {
        let u = vertex.get_index();
        if u < self.len() && vertex == &self[u] {
            Some(u)
        } else {
            None
        }
    }

    pub fn add_vertex(&mut self, v: &mut Vertex<T>) {
        let n = self.len();
        v.reset_index(n);
        self.v_lst.push(v.clone());
        self.e_lst.push(Default::default());
    }

    pub fn add_edge_directed(&mut self, u: &Vertex<T>, v: &Vertex<T>, w: W) -> bool {
        let i = u.get_index();
        if i < self.len() {
            let j = v.get_index();
            if j < self.len() {
                self.e_lst[i].insert(j, w);
                return true;
            }
        }
        false
    }

    pub fn add_edge_undirected(&mut self, u: &Vertex<T>, v: &Vertex<T>, w: W) -> bool {
        let i = u.get_index();
        if i < self.len() {
            let j = v.get_index();
            if j < self.len() {
                self.e_lst[i].insert(j, w);
                self.e_lst[j].insert(i, w);
                return true;
            }
        }
        false
    }

    pub fn add_rev_edges(&mut self) {
        for (u, v, w) in self.iter_edges() {
            self.e_lst[v].insert(u, w);
        }
    }

    pub fn make_rev_e_lst(&self) -> Vec<HashMap<usize, W>> {
        let mut rev_e_lst = vec![HashMap::<usize, W>::new(); self.len()];
        for (u, v, w) in self.iter_edges() {
            rev_e_lst[v].insert(u, w);
        }
        rev_e_lst
    }

    pub fn make_undirected_edges(&self) -> Vec<HashMap<usize, W>> {
        let mut undirected_edges = self.make_rev_e_lst();
        for (u, v, w) in self.iter_edges() {
            undirected_edges[u].insert(v, w);
        }
        undirected_edges
    }

    pub fn make_rev_graph(&self) -> Self {
        Self {
            v_lst: self.v_lst.clone(),
            e_lst: self.make_rev_e_lst(),
        }
    }
}

/// Indexing: you can use g[i] to index vertex
impl<T, W: Weight> Index<usize> for Graph<T, W> {
    type Output = Vertex<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.v_lst.index(index)
    }
}

/// A macro to make vertices only label without carrying any id or data
/// after a graph is made, if you want to change id,
/// you should both change the id in vertex and the index in graph's v_lst
#[macro_export]
macro_rules! add_vertices {
    ($graph:ident # $($vertex:ident),*) => {
	$(
	    let mut $vertex = Vertex::new(stringify!($vertex));
	    $graph.add_vertex(&mut $vertex);
	)*
    }
}

#[macro_export]
macro_rules! add_vertices_with_data {
    ($graph:ident # $($vertex:ident, $data:expr);*) => {
	$(
	    let mut $vertex = Vertex::new_with_data(stringify!($vertex), $data);
	    $graph.add_vertex(&mut $vertex);
	)*
    }
}

// #[macro_export]
// macro_rules! extract_vertices {
//     ($graph_iter:ident # $($vertex:ident),*) => {
// 	$(
// 	    let $vertex = $graph_iter.next().unwrap().clone();
// 	)*
//     }
// }

#[macro_export]
macro_rules! add_unweighted_edges {
    ($graph:ident # $($from:ident: $($to:ident),+);*) => {
	{
	    $(
		let u = $from.clone();
		$(
		    $graph.add_edge_directed(&u, &$to.clone(), crate::NoWeight);
		)+
	    )*
	}
    };
}

#[macro_export]
macro_rules! add_weighted_edges {
    ($graph:ident # $($from:ident: $(($to:ident, $weight:expr)),+);*) => {
	{
	    $(
		let u = $from.clone();
		$(
		    $graph.add_edge_directed(&u, &$to.clone(), $weight);
		)+
	    )*
	}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_unweighted_graph() {
        let mut g1: Graph<(), _> = Graph::new();
        add_vertices!(g1 # a, b, c, d, e, f, g, h, i);
        add_unweighted_edges!(g1 #
            a: b, c;
            b: c, e, i;
            c: d;
            d: a, h;
            e: f;
            f: g;
            g: e, i;
            h: i;
	    i: h);
        assert_eq!(i.get_index(), 8);
        dbg!(&g1);
    }

    #[test]
    fn test_make_weighted_graph() {
        let mut g2: Graph<(), _> = Graph::new();
        add_vertices!(g2 # a, b, c, d, e, f, g, h, i);
        add_weighted_edges!(g2 #
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 10);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7));
        assert_eq!(i.get_index(), 8);
        dbg!(&g2);
    }
}
