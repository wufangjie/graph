use crate::{Graph, Vertex, Weight};
use std::collections::{HashMap, VecDeque};

impl<T, W: Weight> Graph<T, W> {
    /// O(VE^2) find an augmenting path cost E
    /// and will at most augment VE times (the proof is a bit hard)
    /// each augment, we at least make an edge to its capacity (cost E)
    /// why dfs is not ok, see
    /// https://www.zhihu.com/question/38281136/answer/88295342
    /// NOTE1: 对角线是（上下，左右）交替出现
    /// NOTE2: 所有边的方向都是 左->右 上->下
    pub fn edmonds_karp(
        &self,
        start: &Vertex<T>,
        target: &Vertex<T>,
    ) -> HashMap<usize, HashMap<usize, W>> {
        let s = self.get_index_of(start).expect("s not in this graph");
        let t = self.get_index_of(target).expect("t not in this graph");

        let mut matching = HashMap::new();
        while self.edmonds_karp_augment(&mut matching, s, t) {}
        matching
    }

    fn edmonds_karp_augment(
        &self,
        matching: &mut HashMap<usize, HashMap<usize, W>>,
        s: usize,
        t: usize,
    ) -> bool {
        // step1: find augmenting path
        let zero = Default::default();
        let mut queue = VecDeque::new(); // because we can not use inf
        let mut path: HashMap<usize, usize> = HashMap::new();
        for (v, w) in self.iter_edges_from(s) {
            let left = w - get_weight_in(matching, v, s);
            if left != zero {
                queue.push_back((v, left));
                path.insert(v, s);
            }
        }
        let mut w_add = zero;

        while let Some((u, w_max)) = queue.pop_front() {
            for (v, w) in self.iter_edges_from(u) {
                if let std::collections::hash_map::Entry::Vacant(e) = path.entry(v) {
                    // clippy taught me this
                    //if !path.contains_key(&v) {
                    let left = w - get_weight_in(matching, v, u);
                    if left != zero {
                        queue.push_back((v, min(w_max, left)));
                        //path.insert(v, u);
                        e.insert(u);
                        if v == t {
                            w_add = min(w_max, left);
                            break;
                        }
                    }
                }
            }
            if path.contains_key(&t) {
                break;
            }
            if let Some(out) = matching.get(&u) {
                for (&v, &w) in out.iter() {
                    path.entry(v).or_insert_with(|| {
                        queue.push_back((v, min(w_max, w)));
                        u
                    });
                }
            }
        }

        // step2: augment
        if w_add != zero {
            let mut v = t;
            while v != s {
                let u = *path.get(&v).unwrap();

                let to_u = matching.entry(u).or_insert_with(HashMap::new);
                if let Some(&w_v2u) = to_u.get(&v) {
                    if w_v2u == w_add {
                        to_u.remove(&v);
                    } else {
                        to_u.insert(v, w_add - w_v2u);
                    }
                } else {
                    let to_v = matching.entry(v).or_insert_with(HashMap::new);
                    let u2v = to_v.entry(u).or_insert_with(Default::default);
                    *u2v += w_add;
                }
                v = u;
            }
            true
        } else {
            false
        }
    }
}

// fn max<W: Weight>(x: W, y: W) -> W {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }

fn min<W: Weight>(x: W, y: W) -> W {
    if x > y {
        y
    } else {
        x
    }
}

/// return value in matching[u][v]
fn get_weight_in<W: Weight>(matching: &HashMap<usize, HashMap<usize, W>>, u: usize, v: usize) -> W {
    let zero = Default::default();
    if let Some(dct) = matching.get(&u) {
        if let Some(w) = dct.get(&v) {
            *w
        } else {
            zero
        }
    } else {
        zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices, add_weighted_edges};

    #[test]
    fn test_edmonds_karp() {
        let mut g8: Graph<(), i32> = Graph::new();
        add_vertices!(g8 # s, v1, v2, v3, v4, t);
        add_weighted_edges!(g8 #
			    s: (v1, 16), (v2, 13);
			    v1: (v3, 12);
			    v2: (v1, 4), (v4, 14);
			    v3: (v2, 9), (t, 20);
			    v4: (v3, 7), (t, 4));

        let matching = g8.edmonds_karp(&s, &t);
        dbg!(&matching);
    }
}
