pub mod menutree;

use std::fs::File;
use std::path::Path;
use petgraph::graph::UnGraph;
use serde::{Serialize, Deserialize};
use crate::menutree::MenuTree;

/// Represents a usable deltav map
///
/// Can be deserialized from a json value like this:
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
///                   "id": 0
///                 }
///               },
///               {
///                 "EndNode": {
///                   "name": "Node2",
///                   "id": 1
///                 }
///               }
///             ]
///           }
///         },
///         {
///           "EndNode": {
///             "name": "Node3",
///             "id": 2
///           }
///         },
///         {
///           "EndNode": {
///             "name": "Node4",
///             "id": 3
///           }
///         }
///       ]
///     }
///   },
///   "graph": {
///     "nodes": [
///       0,
///       1,
///       2,
///       3
///     ],
///     "node_holes": [],
///     "edge_property": "undirected",
///     "edges": [
///       [
///         0,
///         1,
///         100
///       ],
///       [
///         1,
///         2,
///         400
///       ],
///       [
///         2,
///         3,
///         900
///       ]
///     ]
///   }
/// }
/// ```
#[derive(Deserialize)]
#[cfg_attr(test, derive(Debug, Serialize))]
pub struct DeltavMap {
    menu_tree: MenuTree,
    graph: UnGraph<usize, i32>
}

impl DeltavMap {
    fn get_menu_tree(&self) -> &MenuTree {
        &self.menu_tree
    }

    fn get_graph(&self) -> &UnGraph<usize, i32> {
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
    use petgraph::Graph;
    use petgraph::graph::UnGraph;
    use crate::menutree::tests::get_test_tree;
    use crate::DeltavMap;

    fn get_test_graph() -> UnGraph<usize, i32>{
        let mut graph: UnGraph<usize, i32> = UnGraph::new_undirected();

        let node1 = graph.add_node(0);
        let node2 = graph.add_node(1);
        let node3 = graph.add_node(2);
        let node4 = graph.add_node(3);


        graph.add_edge(node1, node2, 100);
        graph.add_edge(node2, node3, 400);
        graph.add_edge(node3, node4, 900);

        graph
    }

    fn get_test_map() -> DeltavMap {
        let menu_tree = get_test_tree();
        let graph = get_test_graph();

        DeltavMap {menu_tree, graph}
    }

    #[test]
    fn test_deserialize() {
        let file = File::open("res/test.json").unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        let deltav_map: DeltavMap = serde_json::from_value(json).unwrap();

        assert_eq!(deltav_map, get_test_map(), "The deserialized map doesn't equal the test map")
    }
}
