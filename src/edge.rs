/// this module implement three kind of edges
use crate::Weight;

pub trait Edge<W: Weight>: Copy + std::fmt::Debug {
    fn get_from(&self) -> usize;
    fn get_to(&self) -> usize;
    fn get_weight(&self) -> &W;
    fn get_weight_mut(&mut self) -> &mut W;

}

/// most common edge: (from, to, weight) tuple
// type WeightedEdge<W: Weight> = (usize, usize, W); // no bound needed
impl<W: Weight> Edge<W> for (usize, usize, W) {
    fn get_from(&self) -> usize {
        self.0
    }

    fn get_to(&self) -> usize {
        self.1
    }

    fn get_weight(&self) -> &W {
        &self.2
    }

    fn get_weight_mut(&mut self) -> &mut W {
        &mut self.2
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct FlowEdge<W: Weight> {
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) cap: W,
    pub(crate) flow: W,
}

impl<W: Weight> FlowEdge<W> {
    pub fn new(from: usize, to: usize, cap: W) -> Self {
	let flow = Default::default();
        Self {
            from,
            to,
            cap,
            flow,
        }
    }
}

impl<W: Weight> Edge<W> for FlowEdge<W> {
    fn get_from(&self) -> usize {
        self.from
    }

    fn get_to(&self) -> usize {
        self.to
    }

    /// NOTE: it's is cap rather flow
    fn get_weight(&self) -> &W {
        &self.cap
    }

    fn get_weight_mut(&mut self) -> &mut W {
        &mut self.cap
    }

}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct CostFlowEdge<C: Weight, W: Weight> {
    pub(crate) from: usize,
    pub(crate) to: usize,
    pub(crate) cost: C,
    pub(crate) cap: W,
    pub(crate) flow: W,
}

impl<C: Weight, W: Weight> CostFlowEdge<C, W> {
    pub fn new(from: usize, to: usize, cost: C, cap: W) -> Self {
	let flow = Default::default();
        Self {
            from,
            to,
            cost,
            cap,
            flow,
        }
    }
}

impl<C: Weight, W: Weight> Edge<C> for CostFlowEdge<C, W> {
    fn get_from(&self) -> usize {
        self.from
    }

    fn get_to(&self) -> usize {
        self.to
    }

    /// NOTE: CostFlowEdge only need to use cost to find shortest path
    fn get_weight(&self) -> &C {
        &self.cost
    }

    fn get_weight_mut(&mut self) -> &mut C {
        &mut self.cost
    }
}
