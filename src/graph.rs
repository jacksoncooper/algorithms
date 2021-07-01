use std::fs;

fn read_unsigned(maybe_integer: &str) -> usize {
    maybe_integer.parse().expect(
        format!("cannot parse usize from '{}'", maybe_integer).as_str()
    )
}

pub struct Graph {
    vertices: usize,
    edges: usize,
    adjacencies: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            edges: 0,
            adjacencies: vec![Vec::new(); vertices]
        }
    }

    pub fn new_from_file(filename: &str) -> Graph {
        // TODO: Learn how to ? multiple error types instead of panicking.
        // This function presently looks like Christmas morning.

        let description = fs::read_to_string(filename).unwrap();
        let mut lines = description.lines();

        let vertices = read_unsigned(lines.next().unwrap());
        let edges = read_unsigned(lines.next().unwrap());

        let mut graph = Graph::new(vertices);

        for _ in 0..edges {
            let mut integers = lines.next().unwrap().split(' ');
            let from = read_unsigned(integers.next().unwrap());
            let to = read_unsigned(integers.next().unwrap());
            graph.add_edge(from, to);
        }

        graph
    }

    pub fn vertices(&self) -> usize {
        self.vertices
    }

    pub fn edges(&self) -> usize {
        self.edges
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacencies[from].push(to);
        self.adjacencies[to].push(from);
        self.edges += 1;
    }

    pub fn adjacencies(&self, vertex: usize) -> &[usize] {
        &self.adjacencies[vertex]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn read_graph_from_file() {
        let tiny_graph = Graph::new_from_file("./texts/very-tiny-graph.txt");
        assert_eq!(tiny_graph.vertices(), 4);
        assert_eq!(tiny_graph.edges(), 4);
        assert_eq!(tiny_graph.adjacencies(0), vec![1, 0, 0]);
        assert_eq!(tiny_graph.adjacencies(1), vec![0, 3, 3]);
        assert_eq!(tiny_graph.adjacencies(2), vec![]);
        assert_eq!(tiny_graph.adjacencies(3), vec![1, 1]);
    }
}
