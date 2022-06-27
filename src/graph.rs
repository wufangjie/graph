/// This module impl graph's basic data structures and some useful Macros
/// Struct:
/// Graph<T, W>, NOTE: Graph<T, ()> is
/// Macros:
/// 0. make_vertices
/// 1. from_edges_nw
/// 2. from_edges_ww
use crate::Vertex;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

#[derive(Debug)]
pub struct Graph<T, W>
where
    T: Eq + Hash,
    W: Clone + Copy + Default,
{
    pub(crate) v_lst: Vec<Vertex<T>>,
    pub(crate) v_map: HashMap<Vertex<T>, usize>,
    pub(crate) e_lst: Vec<HashMap<usize, W>>, // edges
}

impl<T, W> Graph<T, W>
where
    T: Eq + Hash,
    W: Clone + Copy + Default,
{
    pub fn new() -> Self {
        Self {
            v_lst: Default::default(),
            v_map: Default::default(),
            e_lst: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
	self.v_lst.len()
    }

    pub fn try_insert(&mut self, v: &Vertex<T>) -> usize {
        match self.v_map.get(v) {
            Some(i) => *i,
            None => {
                let i = self.v_lst.len();
                self.v_lst.push(v.clone());
                self.v_map.insert(v.clone(), i);
                self.e_lst.push(Default::default());
                i
            }
        }
    }

    pub fn add_edge_directed(&mut self, u: &Vertex<T>, v: &Vertex<T>, w: W) {
        let i = self.try_insert(u);
        let j = self.try_insert(v);
        self.e_lst[i].insert(j, w);
    }

    pub fn add_edge_undirected(&mut self, u: &Vertex<T>, v: &Vertex<T>, w: W) {
        let i = self.try_insert(u);
        let j = self.try_insert(v);
        self.e_lst[i].insert(j, w);
        self.e_lst[j].insert(i, w);
    }

    // fn add_reverse_edge() {
    // }
}

// fn safe_update<T: Eq + Hash>(
//     dct: &mut HashMap<Vertex<T>, i32>,
//     lst: &mut [Vertex<T>],
//     idx: usize,
//     new_val: T,
// ) {
//     if *lst[idx].borrow() == new_val {
//         return;
//     }
//     let old_opt = dct.remove(&lst[idx]);
//     *lst[idx].borrow_mut() = new_val;
//     if let Some(old) = old_opt {
//         dct.insert(lst[idx].clone(), old);
//     }
// }

#[macro_export]
macro_rules! from_unweighted_edges {
    ($($first:ident: $($rest:ident),+);*) => {
	{
	    let mut g = Graph::new();
	    $(
		let u = $first.clone();
		$(
		    g.add_edge_directed(&u, &$rest.clone(), ());
		)+
	    )*
	    g
	}
    };
}

#[macro_export]
macro_rules! from_weighted_edges {
    ($($first:ident: $(($rest:ident, $weight: expr)),+);*) => {
	{
	    let mut g = Graph::new();
	    $(
		let u = $first.clone();
		$(
		    g.add_edge_directed(&u, &$rest.clone(), $weight);
		)+
	    )*
	    g
	}
    };
}

/// Indexing: you can use g[i] to index vertex
impl<T, W> Index<usize> for Graph<T, W>
where
    T: Eq + Hash,
    W: Clone + Copy + Default,
{
    type Output = Vertex<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.v_lst.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_vertices;
    // fn make_g1() -> Graph<&'static str, ()> {
    // }

    fn make_g2() -> Graph<&'static str, i32> {
        make_vertices!(a, b, c, d, e, f, g, h, i);
        // TODO: maybe keep alphabet order?
        let g2 = from_weighted_edges!(
            a: (b, 4), (h, 8);
            b: (c, 8), (h, 11);
            c: (d, 7), (f, 4), (i, 2);
            d: (e, 9), (f, 14);
            e: (f, 14);
            f: (g, 2);
            g: (h, 1), (i, 6);
            h: (i, 7)
        );
        g2
    }

    #[test]
    fn test_gen() {
        // make unweighted graph
        make_vertices!(a, b, c, d, e, f, g, h, i);
        let g1 = from_unweighted_edges!(
            a: b, c;
            b: c, e, i;
            c: d;
            d: a, h;
            e: f;
            f: g;
            g: e, i;
            h: i;
            i: h
        );
        //dbg!(&g1);
        // indexing
        assert_eq!(g1[0], a);
        assert_eq!(g1[1], b);
        assert_eq!(g1[5], d);
        assert_eq!(*g1.v_map.get(&d).unwrap(), 5);

        // make weighted graph
        let g2 = make_g2();
        //dbg!(&g2);
        // indexing
        assert_eq!(g2[3], c);
        assert_eq!(g2[*g2.v_map.get(&i).unwrap()], i);
    }
}
