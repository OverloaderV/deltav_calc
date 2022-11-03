use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::ops::Index;

/// The menu trees represent nodes in the delta-v map and the categories they are put into
#[derive(Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug, Serialize))]
pub enum MenuTree {
    /// A node representing a category other nodes can be put into
    MiddleNode {
        name: String,
        children: Vec<MenuTree>,
    },

    /// A node holding an index to be used in the graph
    EndNode { name: String, index: NodeIndex },
}

impl MenuTree {
    /// Gets the id of the node. if it's a middle node it panics
    pub fn get_index(&self) -> &NodeIndex {
        match self {
            MenuTree::MiddleNode { .. } => {
                panic!("MiddleNodes don't have indices");
            }
            MenuTree::EndNode { index, .. } => index,
        }
    }

    pub fn get_name(&self) -> &str {
        return match self {
            MenuTree::MiddleNode { name, .. } | MenuTree::EndNode { name, .. } => name,
        };
    }

    pub fn search(&self, search_name: &str) -> Option<&MenuTree> {
        match self {
            MenuTree::EndNode { name, .. } => {
                if name == search_name {
                    Some(self)
                } else {
                    None
                }
            }

            MenuTree::MiddleNode { name, children } => {
                if name == search_name {
                    return Some(self);
                }

                for child in children {
                    let result = child.search(search_name);

                    match result {
                        None => {}
                        Some(_) => {
                            return result;
                        }
                    }
                }

                None
            }
        }
    }
}

impl Index<&str> for MenuTree {
    type Output = MenuTree;

    fn index(&self, index: &str) -> &Self::Output {
        self.search(index).expect("No node with the given name")
    }
}

#[cfg(test)]
pub mod tests {
    use crate::MenuTree;
    use crate::MenuTree::{EndNode, MiddleNode};
    use petgraph::graph::NodeIndex;
    use std::fs::File;
    use std::io::BufReader;

    pub fn get_test_tree() -> MenuTree {
        MiddleNode {
            name: String::from("Category1"),
            children: vec![
                MiddleNode {
                    name: String::from("Category2"),
                    children: vec![
                        EndNode {
                            name: String::from("Node1"),
                            index: NodeIndex::new(0),
                        },
                        EndNode {
                            name: String::from("Node2"),
                            index: NodeIndex::new(1),
                        },
                    ],
                },
                EndNode {
                    name: String::from("Node3"),
                    index: NodeIndex::new(2),
                },
                EndNode {
                    name: String::from("Node4"),
                    index: NodeIndex::new(3),
                },
            ],
        }
    }

    #[test]
    fn test_search() {
        let test_tree = get_test_tree();

        let result = test_tree.search("Category2");
        match result {
            None => {
                panic!("The \"Category2\" node should be found");
            }
            Some(result) => {
                assert_eq!(result.get_name(), "Category2");
            }
        }

        let result = test_tree.search("Node1");
        match result {
            None => {
                panic!("The \"Node1\" node should be found");
            }
            Some(result) => {
                assert_eq!(result.get_name(), "Node1");
                assert_eq!(result.get_index().index(), 0);
            }
        }
    }

    #[test]
    #[should_panic(expected = "MiddleNodes don't have indices")]
    fn test_get_index_panic() {
        get_test_tree().get_index();
    }

    #[test]
    fn test_deserialize() {
        let f = File::open("res/test.json").unwrap();
        let f = BufReader::new(f);
        let json: serde_json::Value = serde_json::from_reader(f).unwrap();
        let json = json.get("menu_tree").unwrap();

        let deserialized: MenuTree = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(deserialized, get_test_tree());
    }

    #[test]
    fn test_index() {
        let test_tree = get_test_tree();

        let result = &test_tree["Category2"];
        assert_eq!(result.get_name(), "Category2");

        let result = &test_tree["Node1"];
        assert_eq!(result.get_name(), "Node1");
        assert_eq!(result.get_index().index(), 0);
    }

    #[test]
    #[should_panic(expected = "No node with the given name")]
    fn test_index_panic() {
        let _ = &get_test_tree()["test"];
    }
}
