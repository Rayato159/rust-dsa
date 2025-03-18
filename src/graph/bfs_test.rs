#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::graph::bfs::bfs;

    #[test]
    fn test_bfs_success_case_1() {
        let graph: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Home", vec!["School", "Train Station", "Moutain"]),
            ("School", vec!["Home", "Train Station", "Mall"]),
            ("Train Station", vec!["Mall", "School", "Home"]),
            ("Mall", vec!["School", "Train Station"]),
            ("Moutain", vec!["Home"]),
        ]);

        let start = "Home";
        let end = "Mall";

        let result = bfs(&graph, start, end);
        assert_eq!(result, true);
        println!();
    }

    #[test]
    fn test_bfs_success_case_2() {
        let graph: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Home", vec!["School", "Train Station", "Moutain"]),
            ("School", vec!["Home", "Train Station", "Mall"]),
            ("Train Station", vec!["Mall", "School", "Home"]),
            ("Mall", vec!["School", "Train Station"]),
            ("Moutain", vec!["Home"]),
        ]);

        let start = "Train Station";
        let end = "Moutain";

        let result = bfs(&graph, start, end);
        assert_eq!(result, true);
        println!();
    }

    #[test]
    fn test_bfs_fail_case_1() {
        let graph: HashMap<&str, Vec<&str>> = HashMap::from([
            ("Home", vec!["School", "Train Station", "Moutain"]),
            ("School", vec!["Home", "Train Station", "Mall"]),
            ("Train Station", vec!["Mall", "School", "Home"]),
            ("Mall", vec!["School", "Train Station"]),
            ("Moutain", vec!["Home"]),
        ]);

        let start = "Train Station";
        let end = "Siam";

        let result = bfs(&graph, start, end);
        assert_eq!(result, false);
        println!();
    }
}
