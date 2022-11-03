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

    fn stock() -> DeltavMap {
        let menu_tree = MiddleNode {name: "Kerbol System".to_owned(), children: vec![
            MiddleNode {name: "Kerbin".to_owned(), children: vec![
                EndNode {name: "Kerbin Landed".to_owned(), id: 0},
                EndNode {name: "Low Kerbin Orbit (80km)".to_owned(), id: 1},
                EndNode {name: "Keostationary Orbit (2863.33km)".to_owned(), id: 2},
                EndNode {name: "Kerbin Intercept".to_owned(), id: 3},
                MiddleNode {name: "Mun".to_owned(), children: vec![
                    EndNode {name: "Mun Intercept".to_owned(), id: 4},
                    EndNode {name: "Low Mun Orbit (14km)".to_owned(), id: 5},
                    EndNode {name: "Mun Landed".to_owned(), id: 6}
                ]},
                MiddleNode {name: "Minmus".to_owned(), children: vec![
                    EndNode {name: "Minmus Intercept".to_owned(), id: 7},
                    EndNode {name: "Low Minmus Orbit (10km)".to_owned(), id: 8},
                    EndNode {name: "Minmus Landed".to_owned(), id: 9}
                ]}
            ]},
            EndNode {name: "Low Kerbol Transfer Orbit (610km - 13,600Mm)".to_owned(), id: 10},
            EndNode {name: "Low Kerbol Orbit (610km)".to_owned(), id: 11},
            EndNode {name: "Kerbol Surface".to_owned(), id: 12},
            MiddleNode {name: "Moho".to_owned(), children: vec![
                EndNode {name: "Moho Intercept".to_owned(), id: 13},
                EndNode {name: "Low Moho Orbit (20km)".to_owned(), id: 14},
                EndNode {name: "Moho Landed".to_owned(), id: 15},
            ]},
            MiddleNode {name: "Eve".to_owned(), children: vec![
                EndNode {name: "Eve Intercept".to_owned(), id: 16},
                EndNode {name: "Eve Capture (100km - 85 Mm)".to_owned(), id: 17},
                EndNode {name: "Low Eve Orbit (100km)".to_owned(), id: 18},
                EndNode {name: "Eve Landed".to_owned(), id: 19},
                MiddleNode {name: "Gilly".to_owned(), children: vec![
                    EndNode {name: "Gilly Intercept".to_owned(), id: 20},
                    EndNode {name: "Low Gilly Orbit (10km)".to_owned(), id: 21},
                    EndNode {name: "Gilly Landed".to_owned(), id: 22},
                ]}
            ]},
            MiddleNode {name: "Duna".to_owned(), children: vec![
                EndNode {name: "Duna Intercept".to_owned(), id: 23},
                EndNode {name: "Duna Capture (60km - 48Mm)".to_owned(), id: 24},
                EndNode {name: "Low Duna Orbit (60km)".to_owned(), id: 25},
                EndNode {name: "Duna Landed".to_owned(), id: 26},
                MiddleNode {name: "Ike".to_owned(), children: vec![
                    EndNode {name: "Ike Intercept".to_owned(), id: 27},
                    EndNode {name: "Low Ike Orbit (10km)".to_owned(), id: 28},
                    EndNode {name: "Ike Landed".to_owned(), id: 29},
                ]}
            ]},
            MiddleNode {name: "Dres".to_owned(), children: vec![
                EndNode {name: "Dres Intercept".to_owned(), id: 30},
                EndNode {name: "Low Dres Orbit (12km)".to_owned(), id: 31},
                EndNode {name: "Dres Landed".to_owned(), id: 32},
            ]}
        ]};

        let mut graph: UnGraph<usize, i32> = UnGraph::new_undirected();
        let nodes = [
            graph.add_node(0),
            graph.add_node(1),
            graph.add_node(2),
            graph.add_node(3),
            graph.add_node(4),
            graph.add_node(5),
            graph.add_node(6),
            graph.add_node(7),
            graph.add_node(8),
            graph.add_node(9),
            graph.add_node(10),
            graph.add_node(11),
            graph.add_node(12),
            graph.add_node(13),
            graph.add_node(14),
            graph.add_node(15),
            graph.add_node(16),
            graph.add_node(17),
            graph.add_node(18),
            graph.add_node(19),
            graph.add_node(20),
            graph.add_node(21),
            graph.add_node(22),
            graph.add_node(23),
            graph.add_node(24),
            graph.add_node(25),
            graph.add_node(26),
            graph.add_node(27),
            graph.add_node(28),
            graph.add_node(29),
            graph.add_node(30),
            graph.add_node(31),
        ];

        graph.add_edge(nodes[0], nodes[1], 3400);
        graph.add_edge(nodes[1], nodes[2], 1115);
        graph.add_edge(nodes[2], nodes[3], 930);
        graph.add_edge(nodes[1], nodes[4], 860);
        graph.add_edge(nodes[4], nodes[5], 280);
        graph.add_edge(nodes[5], nodes[6], 580);
        graph.add_edge(nodes[1], nodes[7], 930);
        graph.add_edge(nodes[7], nodes[8], 150);
        graph.add_edge(nodes[8], nodes[9], 180);
        graph.add_edge(nodes[3], nodes[10], 6000);
        graph.add_edge(nodes[10], nodes[11], 13700);
        graph.add_edge(nodes[11], nodes[12], 67000);
        graph.add_edge(nodes[3], nodes[13], 760);
        graph.add_edge(nodes[13], nodes[14], 2400);
        graph.add_edge(nodes[14], nodes[15], 870);
        graph.add_edge(nodes[16], nodes[17], 80);
        graph.add_edge(nodes[17], nodes[18], 1300);
        graph.add_edge(nodes[18], nodes[19], 8000);
        graph.add_edge(nodes[16], nodes[20], 60);
        graph.add_edge(nodes[20], nodes[21], 410);
        graph.add_edge(nodes[21], nodes[22], 30);
        graph.add_edge(nodes[3], nodes[23], 130);
        graph.add_edge(nodes[23], nodes[24], 250);
        graph.add_edge(nodes[24], nodes[25], 360);
        graph.add_edge(nodes[25], nodes[26], 1450);
        graph.add_edge(nodes[24], nodes[27], 30);
        graph.add_edge(nodes[27], nodes[28], 150);
        graph.add_edge(nodes[28], nodes[29], 150);
        graph.add_edge(nodes[3], nodes[30], 610);
        graph.add_edge(nodes[30], nodes[31], 1300);

        DeltavMap {menu_tree, graph}
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
