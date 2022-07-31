use crate::{Edge, Graph};

/// return partial topological order
/// i.e. res.len() maybe not equal to graph.len()
/// actually VGraph can be faster to find count and init stack
pub fn topo_sort_rc<G: Graph>(graph: &G) -> Vec<usize> {
    let n = graph.len();
    let mut count = vec![0; n];
    let mut backward = vec![vec![]; n];
    for e in graph.iter_e_all() {
        count[e.from()] += 1;
        backward[e.to()].push(e.from());
    }

    let mut stack = vec![];
    for (i, c) in count.iter().enumerate() {
        if *c == 0 {
            stack.push(i);
        }
    }

    let mut res: Vec<usize> = Vec::with_capacity(n);
    while let Some(u) = stack.pop() {
        res.push(u);
        for &v in backward[u].iter() {
            count[v] -= 1;
            if count[v] == 0 {
                stack.push(v);
            }
        }
    }
    res
}

/// It's a special kind of DFS, which yield vertex after all it's out degrees been visited
/// return: since we always need the entire order, Vec is better than Iterator
/// cycle: this implement can processs graph which are not DAG (without dead loop)
/// this implement only promise: **at least one vertex** in scc occur after all scc's out degrees
/// this stack version implement, vertex may in stack multiple times
/// before popping, those vertices in stack just the same as those not processsed loop (dfs)
pub fn topo_sort_dfs<G: Graph>(graph: &G) -> Vec<usize> {
    let n = graph.len();
    let mut stack = vec![0];
    let mut visited = vec![0; n];
    let mut index = 1;
    let mut res = Vec::with_capacity(n);

    loop {
        if let Some(u) = stack.pop() {
            if visited[u] < 2 {
                visited[u] += 1;
                if visited[u] == 2 {
                    res.push(u);
                } else {
                    stack.push(u);
                    for v in graph.iter_v_from(u) {
                        if visited[v] == 0 {
                            stack.push(v);
                        }
                    }
                }
            }
        } else if res.len() == n {
            return res;
        } else {
            while visited[index] != 0 {
                index += 1;
            }
            stack.push(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakeGraph;

    #[test]
    fn test_topo_sort_rc() {
        let g = MakeGraph::mst(false);
        for v in topo_sort_rc(&g) {
            dbg!(v);
        }
    }

    #[test]
    fn test_topo_sort_dfs() {
        let g = MakeGraph::scc();
        for v in topo_sort_dfs(&g) {
            dbg!(v);
        }
    }
}
