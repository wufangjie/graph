use crate::{Graph, Vertex, Weight};
use std::collections::{HashMap, HashSet};

impl<T, W: Weight> Graph<T, W> {
    /// you can get the count of edge disjoint path by: matching[t].len()
    /// you also can get one paths solution: see #[test]
    pub fn edge_disjoint_path(
        &self,
        start: &Vertex<T>,
        target: &Vertex<T>,
    ) -> HashMap<usize, HashSet<usize>> {
        let s = self.get_index_of(start).expect("s not in this graph");
        let t = self.get_index_of(target).expect("t not in this graph");

        let mut matching = HashMap::new();
        while self.edge_disjoint_augment(&mut matching, s, t) {}
        matching
    }

    fn edge_disjoint_augment(
        &self,
        matching: &mut HashMap<usize, HashSet<usize>>,
        s: usize,
        t: usize,
    ) -> bool {
        // step1: find augmenting path
        let mut stack = vec![s];
        let mut path = HashMap::new();

        while let Some(u) = stack.pop() {
            for &v in self.e_lst[u].keys() {
                let exist = if let Some(out) = matching.get(&v) {
                    out.contains(&u)
                } else {
                    false
                };
                if !exist {
                    path.entry(v).or_insert_with(|| {
                        stack.push(v);
                        u
                    });
                    if v == t {
                        break;
                    }
                }
            }
            if path.contains_key(&t) {
                break;
            }
            if let Some(out) = matching.get(&u) {
                for &v in out.iter() {
                    path.entry(v).or_insert_with(|| {
                        stack.push(v);
                        u
                    });
                }
            }
        }

        // step2: augment
        if path.contains_key(&t) {
            let mut v = t;
            while v != s {
                let u = *path.get(&v).unwrap();

                let out = matching.entry(v).or_insert_with(HashSet::new);
                if out.contains(&u) {
                    out.remove(&u);
                } else {
                    out.insert(u);
                }
                v = u;
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_unweighted_edges, add_vertices, Vertex};

    #[test]
    fn test_edge_disjoint_path() {
        let mut g7: Graph<(), _> = Graph::new();
        add_vertices!(g7 # s, a, b, c, d, e, t);
        add_unweighted_edges!(g7 #
            s: a, c, e;
            a: b;
            b: t;
            c: b, d, t;
            d: t;
            e: c);

        let matching = g7.edge_disjoint_path(&s, &t);
        dbg!(&matching);

        if !matching.is_empty() {
            let mut used = HashSet::new();
            let ti = t.get_index();
            let si = s.get_index();
            for &v in matching.get(&ti).unwrap() {
                let mut res = vec![&t, &g7[v]];
                let mut v = v;
                loop {
                    for &u in matching.get(&v).unwrap() {
                        if !used.contains(&(v, u)) {
                            res.push(&g7[u]);
                            used.insert((v, u));
                            v = u;
                            break;
                        }
                    }
                    if v == si {
                        break;
                    }
                }
                dbg!(res);
            }
        }
    }
}
