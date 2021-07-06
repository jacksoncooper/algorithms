use std::collections;

use crate::graphs::Graph;

pub struct Path {
    source: usize,
    marked: Vec<bool>,
    came_from: Vec<Option<usize>>,
}

enum Traversal {
    Depth,
    Breadth
}

impl Path {
    pub fn new_using_depth(graph: &Graph, source: usize) -> Path {
        Path::new(graph, source, Traversal::Depth)
    }

    pub fn new_using_breadth(graph: &Graph, source: usize) -> Path {
        Path::new(graph, source, Traversal::Breadth)
    }

    fn new(graph: &Graph, source: usize, traversal: Traversal) -> Path {
        let mut path = Path {
            source,
            marked: vec![false; graph.vertices()],
            came_from: vec![None; graph.vertices()],
        };

        let mut to_explore = collections::VecDeque::new();
        to_explore.push_back(source);

        // Depth-first search is usually expressed with recursion, but I think
        // the explicit stack makes its behavior more obvious. The vertices
        // most recently discovered are explored first.

        while !to_explore.is_empty() {
            let from = match traversal {
                Traversal::Depth => to_explore.pop_back(),
                Traversal::Breadth => to_explore.pop_front(),
            };

            // Safe to unwrap a nonempty deque. A panic here is a logic error.
            let from = from.unwrap();

            path.marked[from] = true;
            for &to in graph.adjacencies(from) {
                if !path.marked[to] {
                    path.came_from[to] = Some(from);
                    to_explore.push_back(to);
                }
            }
        }

        path
    }

    pub fn has_path_to(&self, vertex: usize) -> bool {
        return self.marked[vertex];
    }

    pub fn path_to(&self, vertex: usize) -> Option<Vec<usize>> {
        if !self.marked[vertex] {
            return None;
        }

        let mut traveler = vertex;
        let mut path: Vec<usize> = Vec::new();

        while traveler != self.source {
            path.push(traveler);
            
            // The given vertex is marked, so its corresponding entry in
            // came_from is not None, except for the source. A panic here is
            // a logic error.

            traveler = self.came_from[traveler].unwrap();
        }

        path.push(self.source);

        // The call to into_iter() consumes 'path'.
        let reverse = path.into_iter().rev().collect();

        return Some(reverse);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_first_search() {
        let tiny = Graph::new_undirected_from_file("./texts/tiny-graph.txt");
        let meander = Path::new_using_depth(&tiny, 0);

        assert!(meander.has_path_to(0));
        assert_eq!(meander.path_to(0), Some(vec![0]));

        assert!(meander.has_path_to(3));
        assert_eq!(meander.path_to(3), Some(vec![0, 6, 4, 5, 3]));

        assert!(!meander.has_path_to(8));
        assert_eq!(meander.path_to(8), None);
    }

    #[test]
    fn breadth_first_search() {
        let tiny = Graph::new_undirected_from_file("./texts/tiny-graph.txt");
        let meander = Path::new_using_breadth(&tiny, 0);

        assert!(meander.has_path_to(0));
        assert_eq!(meander.path_to(0), Some(vec![0]));

        assert!(meander.has_path_to(3));
        assert_eq!(meander.path_to(3), Some(vec![0, 6, 4, 3]));

        assert!(!meander.has_path_to(8));
        assert_eq!(meander.path_to(8), None);
    }
}
