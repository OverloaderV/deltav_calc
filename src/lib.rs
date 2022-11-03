pub mod menutree;

use petgraph::graph::UnGraph;
use serde::{Serialize, Deserialize};
use crate::menutree::MenuTree;

/// Represents a usable deltav map
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Serialize))]
pub struct DeltavMap {
    menu_tree: MenuTree,
    graph: UnGraph<String, i32>
}

impl DeltavMap {
    fn get_menu_tree(&self) -> &MenuTree {
        &self.menu_tree
    }

    fn get_graph(&self) -> &UnGraph<String, i32> {
        &self.graph
    }
}

#[cfg(test)]
impl PartialEq for DeltavMap {
    fn eq(&self, other: &Self) -> bool {
         self.menu_tree == other.menu_tree && format!("{:?}", self.graph) == format!("{:?}", other.graph)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use petgraph::graph::UnGraph;
    use crate::DeltavMap;
    use crate::MenuTree::{EndNode, MiddleNode};


    fn get_test_map() -> DeltavMap {
        let mut graph: UnGraph<String, i32> = UnGraph::new_undirected();

        let menu_tree = MiddleNode { name: "Category1".to_owned(), children: vec![
            MiddleNode { name: "Category2".to_owned(), children: vec![
                EndNode { name: String::from("Node1"), index: graph.add_node(String::from("Node1"))},
                EndNode { name: String::from("Node2"), index: graph.add_node(String::from("Node2"))},
            ] },
            EndNode { name: String::from("Node3"), index: graph.add_node(String::from("Node3")) },
            EndNode { name: String::from("Node4"), index: graph.add_node(String::from("Node4")) },
        ] };

        graph.add_edge(menu_tree["Node1"].get_index().clone(),
                       menu_tree["Node2"].get_index().clone(),
                       900);
        graph.add_edge(menu_tree["Node2"].get_index().clone(),
                       menu_tree["Node3"].get_index().clone(),
                       80);
        graph.add_edge(menu_tree["Node3"].get_index().clone(),
                       menu_tree["Node4"].get_index().clone(),
                       50);

        DeltavMap { menu_tree, graph }
    }

    #[test]
    fn test_deserialize() {
        let file = File::open("res/test.json").unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        let deltav_map: DeltavMap = serde_json::from_value(json).unwrap();

        assert_eq!(deltav_map, get_test_map(), "The deserialized map doesn't equal the test map")
    }
}
