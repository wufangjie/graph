/// One trait: Graph
/// Two kinds of struct which implemnted Graph trait: VGraph, EGraph
use crate::{Edge, Weight};
use std::collections::{HashMap, HashSet};

/// all `u` in this trait's method may >= self.len(), this may panic
/// we do not handle this error for simplicity
pub trait Graph {
    type Edge: Edge;

    /// return the number of vertices
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// iter all the vertices of the graph
    fn iter_v_all(&self) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(0..self.len())
    }

    /// iter all the vertices from vertex `u`
    fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_>;

    /// iter all the vertices to vertex `u`
    fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_>;

    /// this default implemnt use set to promise we only access vertex once
    /// if your graph can promise in and out degrees share no intersection
    /// you can reimplement it like:
    /// Box::new(self.iter_v_from(u).chain(self.iter_v_to(u)))
    fn iter_v_around(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        let mut set: HashSet<usize> = self.iter_v_from(u).collect();
	for v in self.iter_v_to(u) {
	    set.insert(v);
	}
        Box::new(set.into_iter())
    }

    /// iter all the edges of the graph
    fn iter_e_all(&self) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.iter_v_all().flat_map(|u| self.iter_e_from(u)))
    }

    /// iter all the edges from vertex `u`
    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_>;

    /// iter all the edges to vertex `u`
    fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_>;

    /// this default implemnt just is ok, in/out no duplicate edge
    fn iter_e_around(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.iter_e_from(u).chain(self.iter_e_to(u)))
    }
}

/// list (the index present the vertex's id) of map (the vertex's all outdegrees)
/// vertex is first-class element in this struct
pub struct VGraph<W: Weight> {
    lst: Vec<HashMap<usize, W>>,
}

impl<W: Weight> VGraph<W> {
    pub fn new(lst: Vec<HashMap<usize, W>>) -> Self {
        Self { lst }
    }

    // pub fn from_rev_edges<E: Edge>(graph: &impl Graph) -> VGraph<NoWeight> {
    // 	let n = graph.len();
    // 	let mut lst = vec![HashMap::new(); n];
    // 	for e in graph.iter_e_all() {
    //         lst[e.to()].insert(e.from(), NoWeight);
    // 	}
    // 	VGraph::new(lst)
    // }
}

impl<W: Weight> Graph for VGraph<W> {
    type Edge = (usize, usize, W);

    fn len(&self) -> usize {
        self.lst.len()
    }

    fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.lst[u].keys().cloned())
    }

    fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(
            self.iter_v_all()
                .filter(move |v| self.lst[*v].contains_key(&u)),
        )
    }

    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.lst[u].iter().map(move |(v, w)| (u, *v, *w)))
    }

    fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(
            self.iter_v_to(u)
                .map(move |v| (v, u, *self.lst[v].get(&u).unwrap())),
        )
    }
}

/// list (the index present the vertex's id) of map (the vertex's all outdegrees)
/// vertex is first-class element in this struct
/// this data structure is not good at iter vertices
pub struct EGraph<E: Edge> {
    e_lst: Vec<E>,          // all edges
    v_lst: Vec<Vec<usize>>, // ith vertex's all connected edge
}

impl<E: Edge> Graph for EGraph<E> {
    type Edge = E;

    fn len(&self) -> usize {
        self.v_lst.len()
    }

    fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        let set: HashSet<usize> = self.iter_e_from(u).map(|e| e.from()).collect();
        Box::new(set.into_iter())
    }

    fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        let set: HashSet<usize> = self.iter_e_to(u).map(|e| e.to()).collect();
        Box::new(set.into_iter())
    }

    /// implement by hand is faster
    fn iter_v_around(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        let unq: HashSet<usize> = self
            .iter_e_around(u)
            .map(move |e| if e.from() == u { e.to() } else { e.from() })
            .collect();
        Box::new(unq.into_iter())
    }

    /// implement by hand is faster
    fn iter_e_all(&self) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.e_lst.iter().cloned())
    }

    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.iter_e_around(u).filter(move |e| e.from() == u))
    }

    fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.iter_e_around(u).filter(move |e| e.to() == u))
    }

    fn iter_e_around(&self, u: usize) -> Box<dyn Iterator<Item = Self::Edge> + '_> {
        Box::new(self.v_lst[u].iter().map(|i| self.e_lst[*i]))
    }
}

/// A macro to make vertices from 0..n
#[macro_export]
macro_rules! make_vertices {
    ($($var:ident),+) => {
	make_vertices_rec!(0, $($var),+);
    };
}

#[macro_export]
macro_rules! make_vertices_rec {
    ($id:expr, $head:ident, $($tail:ident),*) => {
	let $head = $id;
	make_vertices_rec!($id+1, $($tail),*);
    };
    ($id:expr, $last:ident) => {
	let $last = $id;
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_edge_1() {
        let g = MakeGraph::scc();

        for v in g.iter_v_from(3) {
            dbg!(v);
        }

        println!("-----");
        for v in g.iter_v_to(3) {
            dbg!(v);
        }

        println!("-----");
        for v in g.iter_e_from(3) {
            dbg!(v);
        }

        println!("-----");
        for e in g.iter_e_to(3) {
            dbg!(e);
        }
    }

    #[allow(unused_variables)]
    #[test]
    fn test_macro() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        assert_eq!(i, 8);
    }
}
