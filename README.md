# Rust DSA Mastery Roadmap (สำหรับเตรียมตัว Computer Graphics)

## 1. พื้นฐานโครงสร้างกราฟ (Graph Basics)
- **Graph Representation**: Adjacency List, Matrix
- **Directed vs Undirected Graph**: กราฟมีทิศ/ไม่มีทิศ
- **Weighted vs Unweighted Graph**: กราฟมีน้ำหนักเส้น/ไม่มีน้ำหนัก

## 2. การเดินกราฟ (Traversal Algorithm)
- **DFS (Depth First Search)**: เดินลึกสุดก่อน (หา path, detect cycle, connected component)
- **BFS (Breadth First Search)**: เดินกว้างสุดก่อน (หา shortest path แบบ unweighted)

## 3. การหาเส้นทางสั้นสุด (Shortest Path Algorithm)
- **Dijkstra**: หา shortest path แบบไม่มี weight ติดลบ
- **Bellman-Ford**: หา shortest path ที่มี weight ติดลบได้
- **Floyd-Warshall**: หา shortest path ทุกคู่ node (All-Pairs)

## 4. การหาวงวน (Cycle Detection)
- **Detect Cycle in Undirected Graph**: DFS + parent tracking, Union-Find
- **Detect Cycle in Directed Graph (DAG)**: DFS, Topological Sort

## 5. การเรียงงานตาม dependency (Topological Sort)
- ใช้จัดลำดับงานที่มี dependency, build system, compile order

## 6. Minimum Spanning Tree (MST)
- **Kruskal's Algorithm**: หา MST โดยใช้ Union-Find
- **Prim's Algorithm**: หา MST โดยใช้ Priority Queue (Heap)

## 7. Union-Find (Disjoint Set Union - DSU)
- เช็คว่า node อยู่ในกลุ่มเดียวกันหรือไม่ ใช้ใน Kruskal, Network connection, Cycle detection

## 8. Strongly Connected Components (SCC)
- **Tarjan's Algorithm**: หา subset ที่เชื่อมถึงกันทุกทางใน Directed Graph

## 9. A* Search Algorithm
- Dijkstra + heuristic (เดาว่าตัวไหนใกล้เป้าหมายสุด) ใช้ใน Game AI, Pathfinding

## 10. Flow Network / Max Flow
- **Ford-Fulkerson**: หา max flow ระหว่างต้นกับปลายทาง
- **Edmonds-Karp**: ปรับปรุงจาก Ford-Fulkerson โดยใช้ BFS ให้ดีขึ้น

## สรุปลำดับที่ควรเรียนรู้ (เรียงตามโหด)
1. ✅ Graph Representation (Adjacency List, Matrix)
2. ✅ DFS / BFS
3. ✅ Stack / Queue (พื้นฐาน)
4. ✅ Dijkstra (Shortest Path)
5. Bellman-Ford (Graph ติดลบ)
6. Floyd-Warshall (All-Pairs Shortest Path)
7. Cycle Detection (DFS, Union-Find)
8. Topological Sort (DAG)
9. MST (Kruskal / Prim)
10. Union-Find (Disjoint Set Union)
11. SCC (Tarjan)
12. A* Search Algorithm
13. Flow Network (Max Flow)

## Unit Testing

```bash
cargo test -- --nocapture
```