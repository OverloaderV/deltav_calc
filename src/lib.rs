//! A crate to generate a graph of the popular delta-v maps used in the game Kerbal Space Program.
//! It allows you to do opperations on an immutable [graph](https://docs.rs/petgraph/latest/petgraph/)
//! and get a tree representation of the graphs nodes to be used in menus

mod menutree;

use petgraph::graph::UnGraph;
use serde::{Serialize, Deserialize};
pub use crate::menutree::MenuTree;

/// Represents a usable deltav map
///
/// # Deserialization
/// A DeltavMap can be deserialized from a JSON file like this:
/// ```json
/// {
///   "menu_tree": {
///     "MiddleNode": {
///       "name": "Category1",
///       "children": [
///         {
///           "MiddleNode": {
///             "name": "Category2",
///             "children": [
///               {
///                 "EndNode": {
///                   "name": "Node1",
///                   "index": 0
///                 }
///               },
///               {
///                 "EndNode": {
///                   "name": "Node2",
///                   "index": 1
///                 }
///               }
///             ]
///           }
///         },
///         {
///           "EndNode": {
///             "name": "Node3",
///             "index": 2
///           }
///         },
///         {
///           "EndNode": {
///             "name": "Node4",
///             "index": 3
///           }
///         }
///       ]
///     }
///   },
///   "graph": {
///     "nodes": [
///       "Node1",
///       "Node2",
///       "Node3",
///       "Node4"
///     ],
///     "node_holes": [],
///     "edge_property": "undirected",
///     "edges": [
///       [
///         0,
///         1,
///         900
///       ],
///       [
///         1,
///         2,
///         80
///       ],
///       [
///         2,
///         3,
///         50
///       ]
///     ]
///   }
/// }
/// ```
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Serialize))]
pub struct DeltavMap {
    menu_tree: MenuTree,
    graph: UnGraph<String, i32>
}

impl DeltavMap {
    /// The menu tree you can use to structure your menu
    pub fn get_menu_tree(&self) -> &MenuTree {
        &self.menu_tree
    }

    /// The graph you can use to calculate deltav costs. It's a graph from the [petgraph](https://docs.rs/petgraph/latest/petgraph/) crate
    pub fn get_graph(&self) -> &UnGraph<String, i32> {
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
