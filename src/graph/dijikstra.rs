use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn dijkstra(graph: &HashMap<&str, Vec<(&str, u8)>>, start: &str, end: &str) -> Option<u8> {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<&str, u8> = HashMap::new();

    heap.push(Reverse((0, start)));
    distances.insert(start, 0);

    while let Some(Reverse((cost, current))) = heap.pop() {
        println!("current: {}, cost: {}", current, cost);

        if current == end {
            println!("end: {}, minimum cost: {}", end, cost);
            return Some(cost);
        }

        if let Some(neighbors) = graph.get(current) {
            for (neighbor, weight) in neighbors.iter() {
                println!("neighbor: {}, weight: {}", neighbor, weight);
                let next_cost = cost + weight;

                if distances
                    .get(neighbor)
                    .map_or(true, |&current_cost| next_cost < current_cost)
                {
                    distances.insert(neighbor, next_cost);
                    heap.push(Reverse((next_cost, neighbor)));
                }
            }
        }
    }

    None
}
