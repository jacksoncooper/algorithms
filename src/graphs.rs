pub mod path;

use std::fs;

enum Prefix {
    Undirected,
    Directed,
}

pub struct Graph {
    prefix: Prefix,
    vertices: usize,
    edges: usize,
    adjacencies: Vec<Vec<usize>>,
}

fn read_unsigned(maybe_integer: &str) -> usize {
    maybe_integer.parse().expect(
        format!("cannot parse usize from '{}'", maybe_integer).as_str()
    )
}

impl Graph {
    pub fn new_undirected(vertices: usize) -> Graph {
        return Graph::new(Prefix::Undirected, vertices);
    }

    pub fn new_directed(vertices: usize) -> Graph {
        return Graph::new(Prefix::Directed, vertices);
    }
    
    pub fn new_undirected_from_file(filename: &str) -> Graph {
        return Graph::new_from_file(Prefix::Undirected, filename);
    }

    pub fn new_directed_from_file(filename: &str) -> Graph {
        return Graph::new_from_file(Prefix::Directed, filename);
    }

    fn new(prefix: Prefix, vertices: usize) -> Graph {
        Graph {
            prefix, 
            vertices,
            edges: 0,
            adjacencies: vec![Vec::new(); vertices]
        }
    }

    fn new_from_file(prefix: Prefix, filename: &str) -> Graph {
        // TODO: Learn how to ? multiple error types instead of panicking.
        // This function presently looks like Christmas morning.

        let description = fs::read_to_string(filename).unwrap();
        let mut lines = description.lines();

        let vertices = read_unsigned(lines.next().unwrap());
        let edges = read_unsigned(lines.next().unwrap());

        let mut graph = Graph::new(prefix, vertices);

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

        if self.is_undirected() {
            self.adjacencies[to].push(from);
        }

        self.edges += 1;
    }

    pub fn adjacencies(&self, vertex: usize) -> &[usize] {
        &self.adjacencies[vertex]
    }

    fn is_undirected(&self) -> bool {
        match self.prefix {
            Prefix::Undirected => true,
            Prefix::Directed => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_graph_from_file() {
        let tiny = Graph::new_undirected_from_file("./texts/very-tiny-graph.txt");
        assert_eq!(tiny.vertices(), 4);
        assert_eq!(tiny.edges(), 4);
        assert_eq!(tiny.adjacencies(0), vec![1, 0, 0]);
        assert_eq!(tiny.adjacencies(1), vec![0, 3, 3]);
        assert_eq!(tiny.adjacencies(2), vec![]);
        assert_eq!(tiny.adjacencies(3), vec![1, 1]);
    }
}
