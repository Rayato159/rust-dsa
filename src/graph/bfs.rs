use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> bool {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while !queue.is_empty() {
        if let Some(current) = queue.pop_front() {
            println!("{}", current);

            if current == end {
                return true;
            }

            if let Some(neighbors) = graph.get(current) {
                for neighbor in neighbors.iter() {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        };
    }

    false
}
