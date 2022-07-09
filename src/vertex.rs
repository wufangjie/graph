/// basic data structure: Vertex
/// macro: make_vertices
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Eq, Clone)]
pub struct Vertex<T: Eq + Hash + Clone>(Rc<RefCell<T>>);

impl<T: Eq + Hash + Clone> Vertex<T> {
    pub fn new(x: T) -> Self {
        Self(Rc::new(RefCell::new(x)))
    }

    // pub fn clone(&self) -> Self {
    //     Self(self.0.clone())
    // }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

impl<T: Eq + Hash + Clone> PartialEq for Vertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq + Hash + Clone> Hash for Vertex<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().hash(state);
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> fmt::Display for Vertex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex({})", self.0.borrow())
    }
}

impl<T: Eq + Hash + Clone + fmt::Debug> fmt::Debug for Vertex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex({:?})", self.0.borrow())
    }
}

// impl<T: Eq + Hash + Clone + fmt::Debug> Deref for Vertex<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
// 	&self.0.borrow()
//     }
// }

/// As far as I know, use &str to present a vertex is good enough
/// so I provide this specific macro
#[macro_export]
macro_rules! make_vertices {
    ($($var:ident),*) => {
	$(
            let $var = Vertex::new(stringify!($var));
	)*
    };
}
