use crate::Graph;
use std::collections::VecDeque;

const SENTINEL: usize = usize::MAX;

/// Shortest Path Faster Algorithm
/// prove the correctness of the queue implemention (not stack):
/// 1. every time, in the queue, we can only find two kinds of vertices:
///    within level i/i+1, the shortest distance from s we can expect
///    NOTE: the real distance may be longer (may need more level to reach the distance)
/// 2. suppose we have sentinel between dist i/i+1,
///    the queue's init state: [s, sentinel_0],
///    then if sentinel_i come to the front, we do nothing but pop it and push back a new sentinel_i+1
///    now think about a vertex `u` outqueue, we may have four pushing ways:
///    1: no relaxing, no push
///    2: push its one neighbour `v` to the end of the queue (give it a level i+1 distance, after all level i vertex outqueue, we can get the shortest level i+1 distance)
///    3: update its one neighbour `v` after sentinel, (give it another level i+1 distance)
///    4: update its one neighbour `v` before sentinel (v get a level i+1 distance, after v outqueue, we may achieve more deeper level)
/// NOTE: if we use priorityqueue (distance), sentinel will not work

pub fn spfa<G: Graph>(graph: &G, start: usize) -> (bool, Vec<Option<G::Weight>>, Vec<usize>) {
    let n = graph.len();
    let mut dist = vec![None; n];
    dist[start] = Some(Default::default());
    let mut from = vec![start; n];
    let mut is_in_queue = vec![false; n];

    let mut queue = VecDeque::new();
    queue.push_back(start);
    queue.push_back(SENTINEL);

    let mut level = 0;
    while let Some(u) = queue.pop_front() {
        if u == SENTINEL {
            if queue.is_empty() {
                break;
            } else {
                level += 1;
                if level == n {
                    panic!("Negative cycle existed!");
                }
                queue.push_back(u);
            }
        } else {
            is_in_queue[u] = false;
            let du = dist[u].unwrap();
            for (v, w) in graph.iter_e_from(u) {
                if dist[v].is_none() || du + w < dist[v].unwrap() {
                    from[v] = u;
                    dist[v] = Some(du + w);
                    if !is_in_queue[v] {
                        queue.push_back(v);
                        is_in_queue[v] = true;
                    }
                }
            }
        }
    }
    (true, dist, from)
}

#[cfg(test)]
mod tests {
    use crate::MakeGraph;

    #[test]
    fn test_spfa() {
        let (g, s_lst) = MakeGraph::mst(true);

        let u = 7;
        let (state, dist, from) = g.bellman_ford(u);
        println!("All distance from {}:", s_lst[u]);
        println!("No negative cycle: {}", state);
        for i in 0..dist.len() {
            println!(
                "to: {}, directly from: {}, distance: {:?}",
                s_lst[i], s_lst[from[i]], dist[i]
            )
        }
    }
}
