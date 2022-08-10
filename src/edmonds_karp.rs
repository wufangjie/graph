use crate::{Graph, Weight};
use std::collections::{HashMap, VecDeque};

/// O(VE^2) find an augmenting path cost E
/// and will at most augment VE times (the proof is a bit hard)
/// each augment, we at least make an edge to its capacity (cost E)
/// why dfs is not ok, see
/// https://www.zhihu.com/question/38281136/answer/88295342
/// NOTE1: 对角线是（上下，左右）交替出现
/// NOTE2: 所有边的方向都是 左->右 上->下
pub fn edmonds_karp<G: Graph>(
    graph: &G,
    start: usize,
    target: usize,
) -> HashMap<usize, HashMap<usize, G::Weight>> {
    let mut flowing = HashMap::new();
    while edmonds_karp_augment(graph, &mut flowing, start, target) {}
    flowing
}

fn edmonds_karp_augment<G: Graph>(
    graph: &G,
    flowing: &mut HashMap<usize, HashMap<usize, G::Weight>>,
    start: usize,
    target: usize,
) -> bool {
    // step1: find augmenting path
    let zero = Default::default();
    let mut queue = VecDeque::new(); // because we can not use inf
    let mut path: HashMap<usize, usize> = HashMap::new();
    for (v, w) in graph.iter_e_from(start) {
        let left = w - get_weight_in(flowing, v, start);
        if left != zero {
            queue.push_back((v, left));
            path.insert(v, start);
        }
    }
    let mut w_add = zero;

    while let Some((u, w_max)) = queue.pop_front() {
        for (v, w) in graph.iter_e_from(u) {
            if let std::collections::hash_map::Entry::Vacant(e) = path.entry(v) {
                // clippy taught me this
                //if !path.contains_key(&v) {
                let left = w - get_weight_in(flowing, v, u);
                if left != zero {
                    queue.push_back((v, w_max.min(left)));
                    //path.insert(v, u);
                    e.insert(u);
                    if v == target {
                        w_add = w_max.min(left);
                        break;
                    }
                }
            }
        }
        if path.contains_key(&target) {
            break;
        }
        if let Some(out) = flowing.get(&u) {
            for (&v, &w) in out.iter() {
                path.entry(v).or_insert_with(|| {
                    queue.push_back((v, w_max.min(w)));
                    u
                });
            }
        }
    }

    // step2: augment
    if w_add != zero {
        let mut v = target;
        while v != start {
            let u = *path.get(&v).unwrap();

            let to_u = flowing.entry(u).or_insert_with(HashMap::new);
            if let Some(&w_v2u) = to_u.get(&v) {
                if w_v2u == w_add {
                    to_u.remove(&v);
                } else {
                    to_u.insert(v, w_add - w_v2u);
                }
            } else {
                let to_v = flowing.entry(v).or_insert_with(HashMap::new);
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

/// return value in flowing[u][v]
fn get_weight_in<W: Weight>(flowing: &HashMap<usize, HashMap<usize, W>>, u: usize, v: usize) -> W {
    if let Some(dct) = flowing.get(&u) {
        if let Some(w) = dct.get(&v) {
            return *w;
        }
    }
    Default::default()
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_edmonds_karp() {
        let (g, s_lst) = MakeGraph::mf();
        let (s, t) = (0, 5);
        let flowing = g.edmonds_karp(s, t);
        for (v, dct) in flowing.into_iter() {
            print!("to {}:\tfrom: {{", s_lst[v]);
            let process_tail = !dct.is_empty();
            for (u, w) in dct.into_iter() {
                print!(" {}: {},", s_lst[u], w);
            }
            if process_tail {
                print!("\x08 ");
            }
            println!("}}");
        }
    }
}
