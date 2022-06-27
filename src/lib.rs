pub mod vertex;
pub use vertex::Vertex;

pub mod graph;
pub use graph::Graph;

pub mod dfs;

pub mod bfs;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
