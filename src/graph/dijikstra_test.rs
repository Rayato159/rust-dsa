#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::graph::dijikstra::dijkstra;

    #[test]
    fn test_dijkstra_success_case_1() {
        let graph: HashMap<&str, Vec<(&str, u8)>> = HashMap::from([
            (
                "Home",
                vec![("School", 5), ("Train Station", 10), ("Moutain", 8)],
            ),
            ("School", vec![("Train Station", 3), ("Mall", 7)]),
            ("Train Station", vec![("Mall", 2), ("Home", 10)]),
            ("Moutain", vec![("Home", 8)]),
        ]);

        let start = "Home";
        let end = "Mall";

        let result = dijkstra(&graph, start, end);
        assert_eq!(result, Some(10));
        println!();
    }
}
