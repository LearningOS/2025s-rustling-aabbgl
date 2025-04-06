/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node_a, node_b, weight) = edge;

        // 确保两个节点存在
        self.add_node(node_a);
        self.add_node(node_b);

        // 添加双向边
        let a = node_a.to_string();
        let b = node_b.to_string();

        self.adjacency_table_mutable()
            .entry(a.clone())
            .and_modify(|v| v.push((b.clone(), weight)));

        self.adjacency_table_mutable()
            .entry(b)
            .and_modify(|v| v.push((a, weight)));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        let is_new = !self.adjacency_table().contains_key(node);
        self.adjacency_table_mutable()
            .entry(node.to_string())
            .or_insert(Vec::new());
        is_new
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 默认实现留给具体类型覆盖
        unimplemented!()
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}



#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}