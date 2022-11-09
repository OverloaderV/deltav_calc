//! A crate to generate a graph of the popular delta-v maps used in the game Kerbal Space Program.
//! It allows you to do operations on an immutable [graph](https://docs.rs/petgraph/latest/petgraph/)
//! and get a tree representation of the graphs nodes to be used in menus

extern crate core;

mod menutree;

pub use crate::menutree::{MenuTree, NoSuchNodeError};
use crate::MenuTree::{EndNode, MiddleNode};
use petgraph::algo;
use petgraph::graph::{NodeIndex, UnGraph};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

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
    graph: UnGraph<String, i32>,
}

impl DeltavMap {
    /// The menu tree you can use to structure your menu
    pub fn menu_tree(&self) -> &MenuTree {
        &self.menu_tree
    }

    /// Calculates the deltav required to get from the start to the end
    ///
    /// Returns a [`NoSuchNodeError`] If either start or end aren't valid nodes
    /// Returns `None` if there is no path between nodes. If this happens, the map is probably malformed
    pub fn calculate_delta_v(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Option<i32>, NoSuchNodeError> {
        match self.menu_tree.search(start) {
            Err(e) => Err(e),
            Ok(start) => {
                return match self.menu_tree.search(end) {
                    Err(e) => Err(e),
                    Ok(end) => {
                        let result: Option<(i32, Vec<NodeIndex>)> = algo::astar(
                            &self.graph,
                            start.index().clone(),
                            |finish| finish == end.index().clone(),
                            |e| *e.weight(),
                            |_| 0,
                        );

                        match result {
                            None => Ok(None),
                            Some(result) => Ok(Some(result.0)),
                        }
                    }
                }
            }
        }
    }

    /// Returns a DeltavMap for the stock system
    ///
    /// # Structure of the MenuTree:
    /// ```plain
    /// Kerbol System
    /// ├── Kerbin
    /// │   ├── Kerbin Surface
    /// │   ├── Low Kerbin Orbit (80km)
    /// │   ├── Keostationary Orbit (2.868Mm)
    /// │   ├── Kerbin Capture
    /// │   ├── Mun
    /// │   │   ├── Mun Intercept
    /// │   │   ├── Low Mun Orbit (14km)
    /// │   │   └── Mun Surface
    /// │   └── Minmus
    /// │       ├── Minmus Intercept
    /// │       ├── Low Minmus Orbit (10km)
    /// │       └── Minmus Surface
    /// ├── Eve
    /// │   ├── Eve Intercept
    /// │   ├── Eve Capture (100km - 85Mm)
    /// │   ├── Low Eve Orbit (100km)
    /// │   ├── Eve Surface
    /// │   └── Gilly
    /// │       ├── Gilly Intercept
    /// │       ├── Low Gilly Orbit (10km)
    /// │       └── Gilly Surface
    /// ├── Duna
    /// │   ├── Duna Intercept
    /// │   ├── Duna Capture (60km - 48Mm)
    /// │   ├── Low Duna Orbit (60km)
    /// │   ├── Duna Surface
    /// │   └── Ike
    /// │       ├── Ike Intercept
    /// │       ├── Low Ike Orbit (10km)
    /// │       └── Ike Surface
    /// ├── Jool
    /// │   ├── Jool Intercept
    /// │   ├── Jool Capture (210km - 268Mm)
    /// │   ├── Low Jool Orbit (210km)
    /// │   ├── Jool Surface
    /// │   ├── Pol
    /// │   │   ├── Pol Intercept
    /// │   │   ├── Low Pol Orbit (10km)
    /// │   │   └── Pol Surface
    /// │   ├── Bop
    /// │   │   ├── Bop Intercept
    /// │   │   ├── Low Bop Orbit (30km)
    /// │   │   └── Bop Surface
    /// │   ├── Tylo
    /// │   │   ├── Tylo Intercept
    /// │   │   ├── Low Tylo Orbit (10km)
    /// │   │   └── Tylo Surface
    /// │   ├── Vall
    /// │   │   ├── Vall Intercept
    /// │   │   ├── Low Vall Orbit (15km)
    /// │   │   └── Vall Surface
    /// │   └── Laythe
    /// │       ├── Laythe Intercept
    /// │       ├── Low Laythe Orbit (60km)
    /// │       └── Laythe Surface
    /// ├── Dres
    /// │   ├── Dres Intercept
    /// │   ├── Low Dres Orbit (12km)
    /// │   └── Dres Surface
    /// ├── Moho
    /// │   ├── Moho Intercept
    /// │   ├── Low Moho Orbit (20km)
    /// │   └── Moho Surface
    /// ├── Eeloo
    /// │   ├── Eeloo Intercept
    /// │   ├── Low Eeloo Orbit (10km)
    /// │   └── Eeloo Surface
    /// ├── Elliptical Kerbol Orbit (610km - 13,600Mm)
    /// ├── Low Kerbol Orbit (610km)
    /// └── Kerbol Surface
    /// ```
    pub fn new_stock() -> DeltavMap {
        let mut graph: UnGraph<String, i32> = UnGraph::new_undirected();

        let menu_tree = MiddleNode {
            name: String::from("Kerbol System"),
            children: vec![
                // Kerbin
                MiddleNode {
                    name: String::from("Kerbin"),
                    children: vec![
                        // Surface
                        EndNode {
                            name: String::from("Kerbin Surface"),
                            index: graph.add_node(String::from("Kerbin Surface")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Kerbin Orbit (80km)"),
                            index: graph.add_node(String::from("Low Kerbin Orbit (80km)")),
                        },
                        // Keostationary
                        EndNode {
                            name: String::from("Keostationary Orbit (2.868Mm)"),
                            index: graph.add_node(String::from("Keostationary Orbit (2.868Mm)")),
                        },
                        // Capture
                        EndNode {
                            name: String::from("Kerbin Capture"),
                            index: graph.add_node(String::from("Kerbin Capture")),
                        },
                        // Mun
                        MiddleNode {
                            name: String::from("Mun"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Mun Intercept"),
                                    index: graph.add_node(String::from("Mun Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Mun Orbit (14km)"),
                                    index: graph.add_node(String::from("Low Mun Orbit (14km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Mun Surface"),
                                    index: graph.add_node(String::from("Mun Surface")),
                                },
                            ],
                        },
                        // Minmus
                        MiddleNode {
                            name: String::from("Minmus"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Minmus Intercept"),
                                    index: graph.add_node(String::from("Minmus Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Minmus Orbit (10km)"),
                                    index: graph.add_node(String::from("Low Minmus Orbit (10km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Minmus Surface"),
                                    index: graph.add_node(String::from("Minmus Surface")),
                                },
                            ],
                        },
                    ],
                },
                // Eve
                MiddleNode {
                    name: String::from("Eve"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Eve Intercept"),
                            index: graph.add_node(String::from("Eve Intercept")),
                        },
                        // Capture
                        EndNode {
                            name: String::from("Eve Capture (100km - 85Mm)"),
                            index: graph.add_node(String::from("Eve Capture (100km - 85Mm)")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Eve Orbit (100km)"),
                            index: graph.add_node(String::from("Low Eve Orbit (100km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Eve Surface"),
                            index: graph.add_node(String::from("Eve Surface")),
                        },
                        // Gilly
                        MiddleNode {
                            name: String::from("Gilly"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Gilly Intercept"),
                                    index: graph.add_node(String::from("Gilly Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Gilly Orbit (10km)"),
                                    index: graph.add_node(String::from("Low Gilly Orbit (10km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Gilly Surface"),
                                    index: graph.add_node(String::from("Gilly Surface")),
                                },
                            ],
                        },
                    ],
                },
                // Duna
                MiddleNode {
                    name: String::from("Duna"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Duna Intercept"),
                            index: graph.add_node(String::from("Duna Intercept")),
                        },
                        // Capture
                        EndNode {
                            name: String::from("Duna Capture (60km - 48Mm)"),
                            index: graph.add_node(String::from("Duna Capture (60km - 48Mm)")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Duna Orbit (60km)"),
                            index: graph.add_node(String::from("Low Duna Orbit (60km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Duna Surface"),
                            index: graph.add_node(String::from("Duna Surface)")),
                        },
                        // Ike
                        MiddleNode {
                            name: String::from("Ike"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Ike Intercept"),
                                    index: graph.add_node(String::from("Ike Intercept)")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Ike Orbit (10km)"),
                                    index: graph.add_node(String::from("Low Ike Orbit (10km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Ike Surface"),
                                    index: graph.add_node(String::from("Ike Surface")),
                                },
                            ],
                        },
                    ],
                },
                // Jool
                MiddleNode {
                    name: String::from("Jool"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Jool Intercept"),
                            index: graph.add_node(String::from("Jool Intercept")),
                        },
                        // Capture
                        EndNode {
                            name: String::from("Jool Capture (210km - 268Mm)"),
                            index: graph.add_node(String::from("Jool Capture (210km - 268Mm)")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Jool Orbit (210km)"),
                            index: graph.add_node(String::from("Low Jool Orbit (210km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Jool Surface"),
                            index: graph.add_node(String::from("Jool Surface")),
                        },
                        // Pol
                        MiddleNode {
                            name: String::from("Pol"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Pol Intercept"),
                                    index: graph.add_node(String::from("Pol Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Pol Orbit (10km)"),
                                    index: graph.add_node(String::from("Low Pol Orbit (10km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Pol Surface"),
                                    index: graph.add_node(String::from("Pol Surface")),
                                },
                            ],
                        },
                        // Bop
                        MiddleNode {
                            name: String::from("Bop"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Bop Intercept"),
                                    index: graph.add_node(String::from("Bop Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Bop Orbit (30km)"),
                                    index: graph.add_node(String::from("Low Bop Orbit (30km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Bop Surface"),
                                    index: graph.add_node(String::from("Bop Surface")),
                                },
                            ],
                        },
                        // Tylo
                        MiddleNode {
                            name: String::from("Tylo"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Tylo Intercept"),
                                    index: graph.add_node(String::from("Tylo Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Tylo Orbit (10km)"),
                                    index: graph.add_node(String::from("Low Tylo Orbit (10km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Tylo Surface"),
                                    index: graph.add_node(String::from("Tylo Surface")),
                                },
                            ],
                        },
                        // Vall
                        MiddleNode {
                            name: String::from("Vall"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Vall Intercept"),
                                    index: graph.add_node(String::from("Vall Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Vall Orbit (15km)"),
                                    index: graph.add_node(String::from("Low Vall Orbit (15km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Vall Surface"),
                                    index: graph.add_node(String::from("Vall Surface")),
                                },
                            ],
                        },
                        // Laythe
                        MiddleNode {
                            name: String::from("Laythe"),
                            children: vec![
                                // Intercept
                                EndNode {
                                    name: String::from("Laythe Intercept"),
                                    index: graph.add_node(String::from("Laythe Intercept")),
                                },
                                // Low Orbit
                                EndNode {
                                    name: String::from("Low Laythe Orbit (60km)"),
                                    index: graph.add_node(String::from("Low Laythe Orbit (60km)")),
                                },
                                // Surface
                                EndNode {
                                    name: String::from("Laythe Surface"),
                                    index: graph.add_node(String::from("Laythe Surface")),
                                },
                            ],
                        },
                    ],
                },
                // Dres
                MiddleNode {
                    name: String::from("Dres"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Dres Intercept"),
                            index: graph.add_node(String::from("Dres Intercept")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Dres Orbit (12km)"),
                            index: graph.add_node(String::from("Low Dres Orbit (12km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Dres Surface"),
                            index: graph.add_node(String::from("Dres Surface")),
                        },
                    ],
                },
                // Moho
                MiddleNode {
                    name: String::from("Moho"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Moho Intercept"),
                            index: graph.add_node(String::from("Moho Intercept")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Moho Orbit (20km)"),
                            index: graph.add_node(String::from("Low Moho Orbit (20km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Moho Surface"),
                            index: graph.add_node(String::from("Moho Surface")),
                        },
                    ],
                },
                // Eeloo
                MiddleNode {
                    name: String::from("Eeloo"),
                    children: vec![
                        // Intercept
                        EndNode {
                            name: String::from("Eeloo Intercept"),
                            index: graph.add_node(String::from("Eeloo Intercept")),
                        },
                        // Low Orbit
                        EndNode {
                            name: String::from("Low Eeloo Orbit (10km)"),
                            index: graph.add_node(String::from("Low Eeloo Orbit (10km)")),
                        },
                        // Surface
                        EndNode {
                            name: String::from("Eeloo Surface"),
                            index: graph.add_node(String::from("Eeloo Surface")),
                        },
                    ],
                },
                // Elliptical Orbit
                EndNode {
                    name: String::from("Elliptical Kerbol Orbit (610km - 13,600Mm)"),
                    index: graph
                        .add_node(String::from("Elliptical Kerbol Orbit (610km - 13,600Mm)")),
                },
                // Low Orbit
                EndNode {
                    name: String::from("Low Kerbol Orbit (610km)"),
                    index: graph.add_node(String::from("Low Kerbol Orbit (610km)")),
                },
                // Surface
                EndNode {
                    name: String::from("Kerbol Surface"),
                    index: graph.add_node(String::from("Kerbol Surface")),
                },
            ],
        };

        // region Kerbol
        // region Kerbin
        graph.add_edge(
            menu_tree["Kerbin Surface"].index().clone(),
            menu_tree["Low Kerbin Orbit (80km)"].index().clone(),
            3400,
        );
        graph.add_edge(
            menu_tree["Low Kerbin Orbit (80km)"].index().clone(),
            menu_tree["Keostationary Orbit (2.868Mm)"].index().clone(),
            1115,
        );
        graph.add_edge(
            menu_tree["Low Kerbin Orbit (80km)"].index().clone(),
            menu_tree["Kerbin Capture"].index().clone(),
            950,
        );
        // region Mun
        graph.add_edge(
            menu_tree["Low Kerbin Orbit (80km)"].index().clone(),
            menu_tree["Mun Intercept"].index().clone(),
            860,
        );
        graph.add_edge(
            menu_tree["Mun Intercept"].index().clone(),
            menu_tree["Low Mun Orbit (14km)"].index().clone(),
            280,
        );
        graph.add_edge(
            menu_tree["Low Mun Orbit (14km)"].index().clone(),
            menu_tree["Mun Surface"].index().clone(),
            580,
        );
        // endregion Mun
        // region Minmus
        graph.add_edge(
            menu_tree["Low Kerbin Orbit (80km)"].index().clone(),
            menu_tree["Minmus Intercept"].index().clone(),
            930,
        );
        graph.add_edge(
            menu_tree["Minmus Intercept"].index().clone(),
            menu_tree["Low Minmus Orbit (10km)"].index().clone(),
            160,
        );
        graph.add_edge(
            menu_tree["Low Minmus Orbit (10km)"].index().clone(),
            menu_tree["Minmus Surface"].index().clone(),
            180,
        );
        // endregion Minmus
        // endregion Kerbin
        // region Eve
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Eve Intercept"].index().clone(),
            90,
        );
        graph.add_edge(
            menu_tree["Eve Intercept"].index().clone(),
            menu_tree["Eve Capture (100km - 85Mm)"].index().clone(),
            80,
        );
        graph.add_edge(
            menu_tree["Eve Capture (100km - 85Mm)"].index().clone(),
            menu_tree["Low Eve Orbit (100km)"].index().clone(),
            1350,
        );
        graph.add_edge(
            menu_tree["Low Eve Orbit (100km)"].index().clone(),
            menu_tree["Eve Surface"].index().clone(),
            8000,
        );
        // region Gilly
        graph.add_edge(
            menu_tree["Eve Capture (100km - 85Mm)"].index().clone(),
            menu_tree["Gilly Intercept"].index().clone(),
            60,
        );
        graph.add_edge(
            menu_tree["Gilly Intercept"].index().clone(),
            menu_tree["Low Gilly Orbit (10km)"].index().clone(),
            410,
        );
        graph.add_edge(
            menu_tree["Low Gilly Orbit (10km)"].index().clone(),
            menu_tree["Gilly Surface"].index().clone(),
            30,
        );
        // endregion Gilly
        // endregion Eve
        // region Duna
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Duna Intercept"].index().clone(),
            130,
        );
        graph.add_edge(
            menu_tree["Duna Intercept"].index().clone(),
            menu_tree["Duna Capture (60km - 48Mm)"].index().clone(),
            250,
        );
        graph.add_edge(
            menu_tree["Duna Capture (60km - 48Mm)"].index().clone(),
            menu_tree["Low Duna Orbit (60km)"].index().clone(),
            360,
        );
        graph.add_edge(
            menu_tree["Low Duna Orbit (60km)"].index().clone(),
            menu_tree["Duna Surface"].index().clone(),
            1450,
        );
        // region Ike
        graph.add_edge(
            menu_tree["Duna Capture (60km - 48Mm)"].index().clone(),
            menu_tree["Ike Intercept"].index().clone(),
            30,
        );
        graph.add_edge(
            menu_tree["Ike Intercept"].index().clone(),
            menu_tree["Low Ike Orbit (10km)"].index().clone(),
            180,
        );
        graph.add_edge(
            menu_tree["Low Ike Orbit (10km)"].index().clone(),
            menu_tree["Ike Surface"].index().clone(),
            390,
        );
        // endregion Ike
        // endregion Duna
        // region Jool
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Jool Intercept"].index().clone(),
            980,
        );
        graph.add_edge(
            menu_tree["Jool Intercept"].index().clone(),
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            160,
        );
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Low Jool Orbit (210km)"].index().clone(),
            2810,
        );
        graph.add_edge(
            menu_tree["Low Jool Orbit (210km)"].index().clone(),
            menu_tree["Jool Surface"].index().clone(),
            14000,
        );
        // region Pol
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Pol Intercept"].index().clone(),
            160,
        );
        graph.add_edge(
            menu_tree["Pol Intercept"].index().clone(),
            menu_tree["Low Pol Orbit (10km)"].index().clone(),
            820,
        );
        graph.add_edge(
            menu_tree["Low Pol Orbit (10km)"].index().clone(),
            menu_tree["Pol Surface"].index().clone(),
            130,
        );
        // endregion Pol
        // region Bop
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Bop Intercept"].index().clone(),
            220,
        );
        graph.add_edge(
            menu_tree["Bop Intercept"].index().clone(),
            menu_tree["Low Bop Orbit (30km)"].index().clone(),
            900,
        );
        graph.add_edge(
            menu_tree["Low Bop Orbit (30km)"].index().clone(),
            menu_tree["Bop Surface"].index().clone(),
            230,
        );
        // endregion Bop
        // region Tylo
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Tylo Intercept"].index().clone(),
            400,
        );
        graph.add_edge(
            menu_tree["Tylo Intercept"].index().clone(),
            menu_tree["Low Tylo Orbit (10km)"].index().clone(),
            1100,
        );
        graph.add_edge(
            menu_tree["Low Tylo Orbit (10km)"].index().clone(),
            menu_tree["Tylo Surface"].index().clone(),
            2270,
        );
        // endregion Tylo
        // region Vall
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Vall Intercept"].index().clone(),
            620,
        );
        graph.add_edge(
            menu_tree["Vall Intercept"].index().clone(),
            menu_tree["Low Vall Orbit (15km)"].index().clone(),
            910,
        );
        graph.add_edge(
            menu_tree["Low Vall Orbit (15km)"].index().clone(),
            menu_tree["Vall Surface"].index().clone(),
            860,
        );
        // endregion Vall
        // region Laythe
        graph.add_edge(
            menu_tree["Jool Capture (210km - 268Mm)"].index().clone(),
            menu_tree["Laythe Intercept"].index().clone(),
            930,
        );
        graph.add_edge(
            menu_tree["Laythe Intercept"].index().clone(),
            menu_tree["Low Laythe Orbit (60km)"].index().clone(),
            1070,
        );
        graph.add_edge(
            menu_tree["Low Laythe Orbit (60km)"].index().clone(),
            menu_tree["Laythe Surface"].index().clone(),
            2900,
        );
        // endregion Vall
        // endregion Jool
        // region Dres
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Dres Intercept"].index().clone(),
            610,
        );
        graph.add_edge(
            menu_tree["Dres Intercept"].index().clone(),
            menu_tree["Low Dres Orbit (12km)"].index().clone(),
            1290,
        );
        graph.add_edge(
            menu_tree["Low Dres Orbit (12km)"].index().clone(),
            menu_tree["Dres Surface"].index().clone(),
            430,
        );
        // endregion Dres
        // region Moho
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Moho Intercept"].index().clone(),
            760,
        );
        graph.add_edge(
            menu_tree["Moho Intercept"].index().clone(),
            menu_tree["Low Moho Orbit (20km)"].index().clone(),
            2410,
        );
        graph.add_edge(
            menu_tree["Low Moho Orbit (20km)"].index().clone(),
            menu_tree["Moho Surface"].index().clone(),
            870,
        );
        // endregion Moho
        // region Eeloo
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Eeloo Intercept"].index().clone(),
            1140,
        );
        graph.add_edge(
            menu_tree["Eeloo Intercept"].index().clone(),
            menu_tree["Low Eeloo Orbit (10km)"].index().clone(),
            1370,
        );
        graph.add_edge(
            menu_tree["Low Eeloo Orbit (10km)"].index().clone(),
            menu_tree["Eeloo Surface"].index().clone(),
            620,
        );
        // endregion Moho
        graph.add_edge(
            menu_tree["Kerbin Capture"].index().clone(),
            menu_tree["Elliptical Kerbol Orbit (610km - 13,600Mm)"]
                .index()
                .clone(),
            6000,
        );
        graph.add_edge(
            menu_tree["Elliptical Kerbol Orbit (610km - 13,600Mm)"]
                .index()
                .clone(),
            menu_tree["Low Kerbol Orbit (610km)"].index().clone(),
            13700,
        );
        graph.add_edge(
            menu_tree["Low Kerbol Orbit (610km)"].index().clone(),
            menu_tree["Kerbol Surface"].index().clone(),
            67000,
        );
        // endregion Kerbol

        DeltavMap { menu_tree, graph }
    }
}

#[cfg(test)]
impl PartialEq for DeltavMap {
    fn eq(&self, other: &Self) -> bool {
        self.menu_tree == other.menu_tree
            && format!("{:?}", self.graph) == format!("{:?}", other.graph)
    }
}

#[cfg(test)]
mod tests {
    use crate::DeltavMap;
    use crate::MenuTree::{EndNode, MiddleNode};
    use petgraph::graph::UnGraph;
    use std::fs::File;

    fn get_test_map() -> DeltavMap {
        let mut graph: UnGraph<String, i32> = UnGraph::new_undirected();

        let menu_tree = MiddleNode {
            name: "Category1".to_owned(),
            children: vec![
                MiddleNode {
                    name: "Category2".to_owned(),
                    children: vec![
                        EndNode {
                            name: String::from("Node1"),
                            index: graph.add_node(String::from("Node1")),
                        },
                        EndNode {
                            name: String::from("Node2"),
                            index: graph.add_node(String::from("Node2")),
                        },
                    ],
                },
                EndNode {
                    name: String::from("Node3"),
                    index: graph.add_node(String::from("Node3")),
                },
                EndNode {
                    name: String::from("Node4"),
                    index: graph.add_node(String::from("Node4")),
                },
            ],
        };

        graph.add_edge(
            menu_tree["Node1"].index().clone(),
            menu_tree["Node2"].index().clone(),
            900,
        );
        graph.add_edge(
            menu_tree["Node2"].index().clone(),
            menu_tree["Node3"].index().clone(),
            80,
        );
        graph.add_edge(
            menu_tree["Node3"].index().clone(),
            menu_tree["Node4"].index().clone(),
            50,
        );

        DeltavMap { menu_tree, graph }
    }

    #[test]
    fn test_deserialize() {
        let file = File::open("test_res/test.json").unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        let deltav_map: DeltavMap = serde_json::from_value(json).unwrap();

        assert_eq!(
            deltav_map,
            get_test_map(),
            "The deserialized map doesn't equal the test map"
        )
    }

    #[test]
    fn test_stock() {
        let _ = DeltavMap::new_stock();
    }

    #[test]
    fn calculate_cost() {
        let test_map = get_test_map();
        let cost = test_map
            .calculate_delta_v("Node1", "Node4")
            .unwrap()
            .unwrap();

        assert_eq!(cost, 1030);
    }
}
