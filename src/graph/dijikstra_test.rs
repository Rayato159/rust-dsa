#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::graph::dijikstra::dijkstra;

    #[test]
    fn test_dijkstra_success_case_1() {
        let graph: HashMap<&str, Vec<(&str, u8)>> = HashMap::from([
            (
                "Home",
                vec![("School", 5), ("Train Station", 10), ("Mall", 7)],
            ),
            (
                "School",
                vec![("Home", 5), ("Train Station", 3), ("Mall", 10)],
            ),
            (
                "Train Station",
                vec![("Home", 10), ("School", 3), ("Mall", 2), ("Mountain", 2)],
            ),
            (
                "Mall",
                vec![
                    ("Home", 7),
                    ("School", 10),
                    ("Train Station", 2),
                    ("Mountain", 8),
                ],
            ),
            ("Mountain", vec![("Train Station", 2), ("Mall", 8)]),
        ]);

        let start = "Home";
        let end = "Mall";

        let result = dijkstra(&graph, start, end);
        assert_eq!(result, Some(7));
        println!();
    }

    #[test]
    fn test_dijkstra_success_case_2() {
        let graph: HashMap<&str, Vec<(&str, u8)>> = HashMap::from([
            (
                "Home",
                vec![("School", 5), ("Train Station", 10), ("Mall", 7)],
            ),
            (
                "School",
                vec![("Home", 5), ("Train Station", 3), ("Mall", 10)],
            ),
            (
                "Train Station",
                vec![("Home", 10), ("School", 3), ("Mall", 2), ("Mountain", 2)],
            ),
            (
                "Mall",
                vec![
                    ("Home", 7),
                    ("School", 10),
                    ("Train Station", 2),
                    ("Mountain", 8),
                ],
            ),
            ("Mountain", vec![("Train Station", 2), ("Mall", 8)]),
        ]);

        let start = "Mall";
        let end = "School";

        let result = dijkstra(&graph, start, end);
        assert_eq!(result, Some(5));
        println!();
    }

    #[test]
    fn test_dijkstra_success_case_3() {
        let graph: HashMap<&str, Vec<(&str, u8)>> = HashMap::from([
            (
                "Home",
                vec![("School", 5), ("Train Station", 10), ("Mall", 7)],
            ),
            (
                "School",
                vec![("Home", 5), ("Train Station", 3), ("Mall", 10)],
            ),
            (
                "Train Station",
                vec![("Home", 10), ("School", 3), ("Mall", 2), ("Mountain", 2)],
            ),
            (
                "Mall",
                vec![
                    ("Home", 7),
                    ("School", 10),
                    ("Train Station", 2),
                    ("Mountain", 8),
                ],
            ),
            ("Mountain", vec![("Train Station", 2), ("Mall", 8)]),
        ]);

        let start = "School";
        let end = "Mountain";

        let result = dijkstra(&graph, start, end);
        assert_eq!(result, Some(5));
        println!();
    }
}
