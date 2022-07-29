use crate::{Graph, Vertex, Weight};
use std::collections::VecDeque;

const SENTINEL: usize = usize::MAX;

impl<T, W: Weight> Graph<T, W> {
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

    pub fn spfa(&self, start: &Vertex<T>) -> (bool, Vec<Option<W>>, Vec<usize>) {
        let s = self
            .get_index_of(start)
            .expect("Start vertex not in this graph");
        let n = self.len();

        let mut dist = vec![None; n];
        dist[s] = Some(Default::default());
        let mut from = vec![s; n];
        let mut inqueue = vec![false; n];

        let mut queue = VecDeque::new();
        queue.push_back(s);
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
                inqueue[u] = false;
                let du = dist[u].unwrap();
                for (v, w) in self.iter_edges_from(u) {
                    if dist[v].is_none() || du + w < dist[v].unwrap() {
                        from[v] = u;
                        dist[v] = Some(du + w);
                        if !inqueue[v] {
                            queue.push_back(v);
                            inqueue[v] = true;
                        }
                    }
                }
            }
        }
        (true, dist, from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{add_vertices, add_weighted_edges};

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P: AsRef<Path>>(filename: P) -> Vec<String> {
        io::BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|x| x.unwrap())
            .collect()
    }

    #[test]
    fn test_spfa() {
        // let rows = read_lines("./data/mediumEWD.txt");
        // let n: usize = rows[0].parse().unwrap();

        // let mut v_lst: Vec<Vertex<()>> = Vec::with_capacity(n);
        // for i in 0..n {
        //     let mut v = Vertex::new(format!("{:0>3}", i));
        //     v.reset_index(i);
        //     v_lst.push(v);
        // }

        // let mut e_lst: Vec<HashMap<usize, f64>> = vec![Default::default(); n];
        // for row in rows[2..].into_iter() {
        //     let u: usize = row[..3].trim().parse().unwrap();
        //     let v: usize = row[4..7].trim().parse().unwrap();
        //     let w: f64 = row[8..].parse().unwrap();
        //     e_lst[u].insert(v, w);
        // }

        // let g = Graph { v_lst, e_lst };

        let mut g: Graph<(), i32> = Graph::new();
        add_vertices!(g # a, b, c, d);
        add_weighted_edges!(g #
                    a: (b, 1);
                    b: (c, 1);
                    c: (d, 1) //; d: (a, -4)
        );

        // let (flag, dist, from) = g.bellman_ford(&g[0]);
        let (_flag2, dist2, _from2) = g.spfa(&g[0]);
        // dbg!(dist == dist2);
        dbg!(dist2);
        //dbg!(dist);
        //dbg!(from);
    }
}
