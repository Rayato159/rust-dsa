#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::graph::dfs::{dfs_recursive, dfs_stack};

    #[test]
    fn test_dfs_stack_success_case_1() {
        let graph: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Home", vec!["School", "Train Station", "Moutain"]),
            ("School", vec!["Train Station", "Mall"]),
            ("Train Station", vec!["Mall", "School", "Home"]),
            ("Mall", vec!["School", "Train Station"]),
            ("Moutain", vec!["Home"]),
        ]);

        let start = "Home";
        let end = "Mall";

        let result = dfs_stack(&graph, start, end);
        assert_eq!(result, true);
        println!();
    }

    #[test]
    fn test_dfs_recursive_success_case_1() {
        let graph: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Home", vec!["School", "Train Station", "Moutain"]),
            ("School", vec!["Train Station", "Mall"]),
            ("Train Station", vec!["Mall", "School", "Home"]),
            ("Mall", vec!["School", "Train Station"]),
            ("Moutain", vec!["Home"]),
        ]);

        let start = "Home";
        let end = "Mall";

        let mut visited: HashMap<&str, bool> = HashMap::new();

        let result = dfs_recursive(&graph, start, end, &mut visited);
        assert_eq!(result, true);
        println!();
    }
}
