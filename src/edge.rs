/// this module implement three kind of edges
use crate::Weight;

pub trait Edge: Copy + std::fmt::Debug {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    //fn weight(&self) -> W;
}

pub trait WeightedEdge<W: Weight>: Edge {
    fn weight(&self) -> W;
}

/// most common edge: (from, to, weight) tuple
// type WeightedEdge<W: Weight> = (usize, usize, W); // no bound needed
impl<W: Weight> Edge for (usize, usize, W) {
    fn from(&self) -> usize {
        self.0
    }

    fn to(&self) -> usize {
        self.1
    }
}

impl<W: Weight> WeightedEdge<W> for (usize, usize, W) {
    fn weight(&self) -> W {
        self.2
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
    pub fn new(from: usize, to: usize, flow: W) -> Self {
	let cap = flow;
        Self {
            from,
            to,
            cap,
            flow,
        }
    }
}

impl<W: Weight> Edge for FlowEdge<W> {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }
}

impl<W: Weight> WeightedEdge<W> for FlowEdge<W> {
    fn weight(&self) -> W {
        self.flow
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct CostFlowEdge<C: Weight, W: Weight> {
    from: usize,
    to: usize,
    cost: C,
    cap: W,
    flow: W,
}

impl<C: Weight, W: Weight> CostFlowEdge<C, W> {
    pub fn new(from: usize, to: usize, cost: C, flow: W) -> Self {
	let cap = flow;
        Self {
            from,
            to,
            cost,
            cap,
            flow,
        }
    }
}

impl<C: Weight, W: Weight> Edge for CostFlowEdge<C, W> {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }
}

impl<C: Weight, W: Weight> WeightedEdge<W> for CostFlowEdge<C, W> {
    fn weight(&self) -> W {
        self.flow
    }
}
