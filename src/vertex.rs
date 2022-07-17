/// basic data structure: Vertex
/// macro: make_vertices
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

/// label: String to present a vertex (for human)
/// index: means index in a graph, usize::MAX means not inited (not in any graph)
/// data: pub field for easy to modify
/// v1 == v2 if and only if their label and index are the same
/// NOTE: most of the time, we don't need this field,
/// just give it a zst: () for example
// #[derive(Clone)] // T can be any type, don't need a Clone bound
pub struct RawVertex<T> {
    label: String,
    index: usize,
    pub data: T,
}

impl<T> RawVertex<T> {
    pub fn new_with_data(label: impl ToString, data: T) -> Self {
        Self {
            label: label.to_string(),
            index: usize::MAX,
            data,
        }
    }
}

impl<T: Default> RawVertex<T> {
    pub fn new(label: impl ToString) -> Self {
        Self::new_with_data(label, Default::default())
    }
}

impl<T> PartialEq for RawVertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.index == other.index
    }
}

/// NOTE: use Rc<RefCell<T>> to keep consistent
//#[derive(Clone, PartialEq)]
pub struct Vertex<T>(Rc<RefCell<RawVertex<T>>>);

impl<T> Vertex<T> {
    pub fn new_with_data(label: impl ToString, data: T) -> Self {
        Self(Rc::new(RefCell::new(RawVertex::new_with_data(label, data))))
    }

    pub fn borrow(&self) -> Ref<'_, RawVertex<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, RawVertex<T>> {
        self.0.borrow_mut()
    }

    pub fn get_index(&self) -> usize {
        self.borrow().index
    }

    /// only suppose to be called when add a vertx to a graph
    /// so I put a &mut before self
    pub(crate) fn reset_index(&mut self, index: usize) {
        self.borrow_mut().index = index;
    }

    // pub(crate) fn get_label(&self) -> String {
    //     self.borrow().label.clone()
    // }
}

impl<T: Default> Vertex<T> {
    pub fn new(label: impl ToString) -> Self {
        Self(Rc::new(RefCell::new(RawVertex::new(label))))
    }
}

impl<T: Default> From<String> for Vertex<T> {
    fn from(label: String) -> Self {
        Self(Rc::new(RefCell::new(RawVertex::new(label))))
    }
}

impl<T> fmt::Display for Vertex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex({})", self.borrow().label)
    }
}

impl<T> fmt::Debug for Vertex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex({:?})", self.borrow().label)
    }
}

/// TODO: why #[derive(Clone)] did not work, I think T: Clone is not necessary
impl<T> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PartialEq for Vertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertices() {
        let v1: Vertex<()> = Vertex::new("a");
        let v2: Vertex<()> = Vertex::new("b");
        let mut v3: Vertex<()> = Vertex::new("b");
        v3.reset_index(1);

        assert_ne!(v1, v2);
        assert_ne!(v2, v3);
        assert_eq!(v1.clone(), v1);
    }
}
