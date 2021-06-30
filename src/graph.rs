use std::fs;

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

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacencies[from].push(to);
        self.adjacencies[to].push(from);
        self.edges += 1;
    }
}

fn read_unsigned(maybe_integer: &str) -> usize {
    maybe_integer.parse().expect(
        format!("cannot parse usize from '{}'", maybe_integer).as_str()
    )
}

#[cfg(test)]
mod tests {
}
