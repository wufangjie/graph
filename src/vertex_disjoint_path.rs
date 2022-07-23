use crate::{Graph, Vertex, Weight};
use std::collections::HashMap;

impl<T, W: Weight> Graph<T, W> {
    /// you can get the count of vertex disjoint path using: last_but_t.len()
    /// you can get one paths solution:
    /// 1. last_but_t keeps all vertice directly to t
    /// 2. matching keeps all sub paths (backward, (k, v) in it means edge v -> k)
    pub fn vertex_disjoint_path(
        &self,
        start: &Vertex<T>,
        target: &Vertex<T>,
    ) -> (HashMap<usize, usize>, Vec<usize>) {
        let s = self.get_index_of(start).expect("s not in this graph");
        let t = self.get_index_of(target).expect("t not in this graph");

        let mut matching = HashMap::new();
        let mut last_but_t = vec![];
        while self.vertex_disjoint_augment(&mut matching, &mut last_but_t, s, t) {}
        (matching, last_but_t)
    }

    fn vertex_disjoint_augment(
        &self,
        matching: &mut HashMap<usize, usize>,
        last_but_t: &mut Vec<usize>,
        s: usize,
        t: usize,
    ) -> bool {
        // step1: find augmenting path
        let mut stack = vec![s];
        let mut path = HashMap::new();
        let mut found = usize::MAX;

        while let Some(u) = stack.pop() {
            if let Some(&v) = matching.get(&u) {
                // every time we can cancel a previous matching
                // NOTE: we always need to update path (v <- u),
                // no matter v has in path or not
		// we may put v into stack twice, but it's ok
                path.insert(v, u);
                stack.push(v);

                // if vertex(u) is in matching, but isn't from cancellation
                // we can not add u's outdegrees
                // NOTE: s will always not in matching, so u != s, unwrap is safe
                if matching.get(path.get(&u).unwrap()) != Some(&u) {
                    continue;
                }
            }
            for v in self.iter_vertices_from(u) {
                if v == t {
                    found = u;
                    break;
                }
                path.entry(v).or_insert_with(|| {
                    stack.push(v);
                    u
                });
            }
        }

        // step2: augment
        if found != usize::MAX {
            last_but_t.push(found);
            let mut v = found;
            while v != s {
                let u = *path.get(&v).unwrap();
		// NOTE: it's important to remove matching when cancel
                if matching.get(&u) == Some(&v) {
                    matching.remove(&u);
                } else {
                    matching.insert(v, u);
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
    fn test_vertex_disjoint_path() {
        let mut g7: Graph<(), _> = Graph::new();
        add_vertices!(g7 # s, a, b, c, d, e, t);
        add_unweighted_edges!(g7 #
            s: a, c, e;
            a: b;
            b: t;
            c: b, d, t;
            d: t;
            e: c);

        let (matching, last_but_t) = g7.vertex_disjoint_path(&s, &t);
        dbg!(&last_but_t);
        dbg!(&matching);

        for mut i in last_but_t {
            let mut res = vec![&t];
            loop {
                let v = &g7[i];
                res.push(v);
                if v == &s {
                    break;
                } else {
                    i = *matching.get(&i).unwrap();
                }
            }
            dbg!(res);
        }
    }
}
