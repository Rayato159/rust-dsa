use std::collections::HashMap;

pub fn dfs_stack(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> bool {
    let mut visited: HashMap<&str, bool> = HashMap::new();
    let mut stack = vec![start];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        println!("{}", current);

        if current == end {
            return true;
        }

        visited.insert(current, true);

        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors.iter().rev() {
                if !visited.contains_key(neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    false
}

pub fn dfs_recursive<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
    visited: &mut HashMap<&'a str, bool>,
) -> bool {
    println!("{}", start);

    if start == end {
        return true;
    }

    visited.insert(start, true);

    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            if !visited.contains_key(neighbor) {
                if dfs_recursive(graph, neighbor, end, visited) {
                    return true;
                }
            }
        }
    }

    false
}
