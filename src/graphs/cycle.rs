use crate::graphs::Graph;

pub fn cycle(graph: &Graph) -> Option<Vec<usize>> {
    let mut marked = vec![false; graph.vertices()];
    let mut exhausted = vec![false; graph.vertices()];
    let mut came_from = vec![None; graph.vertices()];

    for source in 0..graph.vertices() {
        if marked[source] { continue; }

        // Explore the subgraph with an explicit call stack and a Trémaux
        // "string" used to keep track of vertices in the path we're
        // travelling.

        let mut path = vec![source];

        while !path.is_empty() {
            let &from = path.last().unwrap();

            if exhausted[from] {
                exhausted[from] = false;
                path.pop();
                continue;
            }

            marked[from] = true;
            exhausted[from] = true;

            for &to in graph.adjacencies(from) {
                if !marked[to] {
                    came_from[to] = Some(from);
                    path.push(to);
                } else if exhausted[to] {
                    let mut traveler: usize = from;
                    let mut cycle = vec![to];

                    while traveler != to {
                        cycle.push(traveler);
                        traveler = came_from[traveler].unwrap();
                    }
                    cycle.push(to);

                    let reverse = cycle.into_iter().rev().collect();
                    return Some(reverse);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_directed_cycle() {
        let mut tiny = Graph::new_directed(8);

        tiny.add_edge(0, 1);
        tiny.add_edge(2, 4);
        tiny.add_edge(4, 5);
        tiny.add_edge(4, 7);
        tiny.add_edge(5, 3);

        assert!(cycle(&tiny).is_none());

        tiny.add_edge(3, 4);

        assert_eq!(cycle(&tiny).unwrap(), vec![4, 5, 3, 4]);
    }

    #[test]
    fn detect_trivial_directed_cycle() {
        let mut trivial = Graph::new_directed(1);
        assert!(cycle(&trivial).is_none());
        trivial.add_edge(0, 0);
        assert_eq!(cycle(&trivial).unwrap(), vec![0, 0]);
    }

    #[test]
    fn detect_trivial_undirected_cycle() {
        let mut trivial = Graph::new_undirected(2);
        assert!(cycle(&trivial).is_none());
        trivial.add_edge(0, 1);
        assert_eq!(cycle(&trivial).unwrap(), vec![0, 1, 0]);
    }
}
