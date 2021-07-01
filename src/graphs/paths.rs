use crate::graphs::undirected::Undirected;

pub struct Path {
    source: usize,
    marked: Vec<bool>,
    came_from: Vec<Option<usize>>,
}

impl Path {
    pub fn new(source: usize, graph: &Undirected) -> Path {
        let mut path = Path {
            source,
            marked: vec![false; graph.vertices()],
            came_from: vec![None; graph.vertices()],
        };

        let mut to_explore: Vec<usize> = Vec::new();
        to_explore.push(source);

        // Depth-first search is usually expressed with recursion, but I think
        // the explicit stack makes its behavior more obvious. The vertices
        // most recently discovered are explored first.

        while !to_explore.is_empty() {
            // Safe to unwrap a nonempty vector. A panic here is a logic error.
            let from: usize = to_explore.pop().unwrap();
            path.marked[from] = true;
            for &to in graph.adjacencies(from) {
                if !path.marked[to] {
                    path.came_from[to] = Some(from);
                    to_explore.push(to);
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

        return Some(path);
    }
}

