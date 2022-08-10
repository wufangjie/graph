/// One trait: Graph
/// Two kinds of struct which implemnted Graph trait: VGraph, EGraph
/// serveral macros:
//use crate::{Edge, Weight, WeightedEdge, CostFlowEdge};
use crate::Weight;
use std::collections::HashMap;
// use std::marker::PhantomData;

/// all `u` in this trait's method may >= self.len(), this may panic
/// we do not handle this error for simplicity
pub trait Graph {
    type Weight: Weight;

    /// return the number of vertices
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// iter all the vertices from vertex `u`
    fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_>;

    fn is_empty_from(&self, u: usize) -> bool {
        self.iter_v_from(u).next().is_none()
    }

    fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_>;

    fn is_empty_to(&self, u: usize) -> bool {
        self.iter_v_to(u).next().is_none()
    }

    /// iter all the edges from vertex `u`
    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_>;

    // fn iter_mut_e_from(
    //     &mut self,
    //     u: usize,
    // ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_>;

    /// iter all the edges to vertex `u`
    fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_>;

    // fn iter_mut_e_to(
    //     &mut self,
    //     u: usize,
    // ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_>;
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
    type Weight = W;

    fn len(&self) -> usize {
        self.lst.len()
    }

    fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(self.lst[u].keys().cloned())
    }

    /// it's faster than default implement, since no iterator is made?
    fn is_empty_from(&self, u: usize) -> bool {
        self.lst[u].is_empty()
    }

    fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(
            (0..self.len())
                .into_iter()
                .filter(move |v| self.lst[*v].contains_key(&u)),
        )
    }

    /// &Self::Weight -> Self::Weight, we can easily modify (not mutate) the weight
    fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_> {
        Box::new(self.lst[u].iter().map(move |(v, w)| (*v, *w)))
    }

    // fn iter_mut_e_from(
    //     &mut self,
    //     u: usize,
    // ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_> {
    //     Box::new(self.lst[u].iter_mut().map(move |(v, w)| (*v, w)))
    // }

    /// VGraph do not need this
    fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = (usize, Self::Weight)> + '_> {
        Box::new(
            self.iter_v_to(u)
                .map(move |v| (v, *self.lst[v].get(&u).unwrap())),
        )
    }

    // /// VGraph do not need this
    // /// why it's not work?
    // fn iter_mut_e_to(
    //     &mut self,
    //     _u: usize,
    // ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_> {
    //     // Box::new(self.iter_v_to(u).map(|v| (v, self.lst[v].get_mut(&u).unwrap())))
    //     unimplemented!();
    // }
}

// /// edge is the first-class element in this struct,
// /// it is not good at iter vertices (using HashSet to promise uniqueness)
// pub struct EGraph<W, E>
// where
//     W: Weight,
//     E: Edge<W>
// {
//     pub(crate) e_lst: Vec<E>,          // all edges
//     pub(crate) from_v_lst: Vec<Vec<usize>>, // all edge id from v
//     pub(crate) to_v_lst: Vec<Vec<usize>>,   // all edge id to v
//     marker: PhantomData<W>,
// }

// impl<W, E> EGraph<W, E>
// where
//     W: Weight,
//     E: Edge<W>
// {
//     pub fn new(e_lst: Vec<E>, n: usize) -> Self {
// 	let mut from_v_lst = vec![vec![]; n];
// 	let mut to_v_lst = vec![vec![]; n];
// 	for (i, e) in e_lst.iter().enumerate() {
// 	    from_v_lst[e.get_from()].push(i);
// 	    to_v_lst[e.get_to()].push(i);
// 	}
// 	Self { e_lst, from_v_lst, to_v_lst, marker: PhantomData }
//     }

//     pub fn get_weight_mut(&mut self, i: usize) -> &mut W {
// 	self.e_lst[i].get_weight_mut()
//     }
// }

// pub fn len(&self) -> usize {
// 	self.v_lst.len()
// }

//     pub fn iter_e_around(&self, u: usize) -> impl Iterator<Item = E> + '_ {
//         self.v_lst[u].iter().map(|i| self.e_lst[*i])
//     }

//     pub fn iter_e_from(&self, u: usize) -> impl Iterator<Item = E> + '_ {
// 	self.iter_e_around(u).filter(move |e| e.from() == u)
//     }
// }

// impl<W, C> EGraph<W, CostFlowEdge<C, W>>
// where
//     W: Weight,
//     C: Weight,
// {
//     /// iter cost edge
//     pub fn iter_ce_from(&self, u: usize) -> impl Iterator<Item = (usize, usize, C)> + '_ {
// 	self.iter_e_around(u)
// 	    .filter(move |e| e.cap > e.flow)
// 	    .map(move |e| {
// 		if e.from == u {
// 		    (u, e.to, e.cost)
// 		} else {
// 		    (u, e.to, e.cost)
// 		}
// 	    })
//     }
// }

// impl<W, E> Graph for EGraph<W, E>
// where
//     W: Weight,
//     E: Edge<W>
// {
//     type Weight = W;

//     fn len(&self) -> usize {
//         self.from_v_lst.len()
//     }

//     fn iter_v_from(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
// 	!unimplemented!();
//     }

//     fn iter_v_to(&self, u: usize) -> Box<dyn Iterator<Item = usize> + '_> {
// 	!unimplemented!();
//     }

//     fn iter_e_from(&self, u: usize) -> Box<dyn Iterator<Item = (usize, &Self::Weight)> + '_> {
//         Box::new(self.from_v_lst[u].iter().map(move |i| (self.e_lst[*i].get_to(), self.e_lst[*i].get_weight())))
//     }

//     fn iter_mut_e_from(
//         &mut self,
//         u: usize,
//     ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_> {
// 	// Box::new(self.from_v_lst[u].clone().into_iter().map(move |i| (self.e_lst[i].get_to(), self.e_lst[i].get_weight_mut())))
// 	Box::new(EdgeIterMut{e_lst: &mut self.e_lst, i_lst: self.from_v_lst.iter(), index: 0})
//     }

//     fn iter_e_to(&self, u: usize) -> Box<dyn Iterator<Item = (usize, &Self::Weight)> + '_> {
//         Box::new(self.to_v_lst[u].iter().map(move |i| (self.e_lst[*i].get_to(), self.e_lst[*i].get_weight())))
//     }

//     fn iter_mut_e_to(
//         &mut self,
//         u: usize,
//     ) -> Box<dyn Iterator<Item = (usize, &mut Self::Weight)> + '_> {
// 	unimplemented!();
//     }
// }

// struct EdgeIterMut<'a, W, E, I>
// where
//     W: Weight + 'a,
//     E: Edge<W>,
//     I: Iterator<Item = usize>,
// {
//     e_lst: &'a mut Vec<E>,
//     i_lst: &'a I,
//     index: usize,
//     marker: PhantomData<W>
// }

// impl<'a, W, E, I> Iterator for EdgeIterMut<'a, W, E, I>
// where
//     W: Weight,
//     E: Edge<W>,
//     I: Iterator<Item = usize>,
// {
//     type Item = (usize, &'a mut W);

//     fn next(&mut self) -> Option<Self::Item> {
// 	while let Some(i) = self.i_lst.next() {
// 	    return Some((self.e_lst[i].get_to(), self.e_lst[i].get_weight_mut()));
// 	}
// 	None
//     }
// }

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
