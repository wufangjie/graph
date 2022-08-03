/// One trait: Graph
/// Two kinds of struct which implemnted Graph trait: VGraph, EGraph
/// serveral macros:
use crate::{Edge, Weight, WeightedEdge};
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

    fn count_v_from(&self, u: usize) -> usize {
	self.iter_v_from(u).count()
    }

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

/// list of map
/// ith HashMap means: ith vertex's all outdegrees
/// vertex is the first-class element in this struct
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

    /// faster than default
    fn count_v_from(&self, u: usize) -> usize {
	self.lst[u].len()
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

/// edge is the first-class element in this struct,
/// it is not good at iter vertices (using HashSet to promise uniqueness)
pub struct EGraph<W, E>
where
    W: Weight,
    E: WeightedEdge<W>
{
    e_lst: Vec<E>,          // all edges
    v_lst: Vec<Vec<usize>>, // ith vertex's all connected edge
    phantom: std::marker::PhantomData<W>,
}

impl<W, E> EGraph<W, E>
where
    W: Weight,
    E: WeightedEdge<W>
{
    pub fn new(e_lst: Vec<E>, n: usize) -> Self {
	let mut v_lst = vec![vec![]; n];
	for (i, e) in e_lst.iter().enumerate() {
	    v_lst[e.from()].push(i);
	    v_lst[e.to()].push(i);
	}
	Self { e_lst, v_lst, phantom: std::marker::PhantomData }
    }
}

impl<W, E> Graph for EGraph<W, E>
where
    W: Weight,
    E: WeightedEdge<W>
{
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
        let set: HashSet<usize> = self
            .iter_e_around(u)
            .map(move |e| if e.from() == u { e.to() } else { e.from() })
            .collect();
        Box::new(set.into_iter())
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

/// helper macro for make_vertices, needn't call it by hand
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

// #[macro_export]
// macro_rules! make_symbol_lst {
//     ($($var:ident),+) => {
// 	{
// 	    let mut s_lst = vec![];
// 	    $(
// 		s_lst.push(stringify!($var));
// 	    )*
// 	    let set = s_lst.iter().collect::<std::collections::HashSet<_>>();
// 	    if set.len() != s_lst.len() {
// 		println!("Warning: Duplicated Symbols Occured!");
// 	    }
// 	    s_lst
// 	}
//     };
// }

/// make vertex symbols for more ergonomic result presenting
#[macro_export]
macro_rules! make_symbol_lst {
    ($($var:ident),+) => {
	{
	    let s_lst = vec![$(stringify!($var),)*];
	    let set = s_lst.iter().collect::<std::collections::HashSet<_>>();
	    if set.len() != s_lst.len() {
		println!("Warning: Duplicated Symbols Occured!");
	    }
	    s_lst
	}
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_edge_1() {
        let (g, s_lst) = MakeGraph::scc();

        let u = 3;
        println!("iter vertices from: `{}`", s_lst[u]);
        for v in g.iter_v_from(u) {
            dbg!(s_lst[v]);
        }

        println!("iter vertices to: `{}`", s_lst[u]);
        for v in g.iter_v_to(u) {
            dbg!(s_lst[v]);
        }

        println!("iter edges from: `{}`", s_lst[u]);
        for e in g.iter_e_from(u) {
            dbg!(e);
        }

        println!("iter edges to: `{}`", s_lst[u]);
        for e in g.iter_e_to(u) {
            dbg!(e);
        }
    }

    #[allow(unused_variables)]
    #[test]
    fn test_macro() {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        assert_eq!(i, 8);

        let s_lst = make_symbol_lst!(a, b, c, d, e, f, g, h, i, b);
        dbg!(s_lst);
    }
}
