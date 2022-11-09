use petgraph::graph::NodeIndex;
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Index;

/// This error is raised when a node is searched that doesn't exist. It saves the nodes name
#[derive(Debug)]
pub struct NoSuchNodeError {
    name: String,
}

impl NoSuchNodeError {
    pub fn cause_name(&self) -> &str {
        &self.name
    }
}

impl Display for NoSuchNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There is no node with the name \"{}\" in the tree",
            self.name
        )
    }
}

impl Error for NoSuchNodeError {}

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
    pub(crate) fn index(&self) -> &NodeIndex {
        match self {
            MenuTree::MiddleNode { .. } => {
                panic!("MiddleNodes don't have indices");
            }
            MenuTree::EndNode { index, .. } => index,
        }
    }

    pub fn name(&self) -> &str {
        return match self {
            MenuTree::MiddleNode { name, .. } | MenuTree::EndNode { name, .. } => name.as_str(),
        };
    }

    /// Searches for the node with the given name.
    ///
    /// If there is no node with that name, a [`NoSuchNodeError`] will be returned
    pub fn search(&self, search_name: &str) -> Result<&MenuTree, NoSuchNodeError> {
        match self {
            MenuTree::EndNode { name, .. } => {
                if name == search_name {
                    Ok(self)
                } else {
                    Err(NoSuchNodeError {
                        name: search_name.to_string(),
                    })
                }
            }

            MenuTree::MiddleNode { name, children } => {
                if name == search_name {
                    return Ok(self);
                }

                for child in children {
                    let result = child.search(search_name);

                    match result {
                        Ok(_) => return result,
                        Err(_) => {}
                    }
                }

                Err(NoSuchNodeError {
                    name: search_name.to_string(),
                })
            }
        }
    }
}

impl Index<&str> for MenuTree {
    type Output = MenuTree;

    fn index(&self, index: &str) -> &Self::Output {
        match self.search(index) {
            Ok(result) => result,
            Err(e) => {
                panic!("{e}")
            }
        }
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
            Err(e) => {
                panic!("{}", e)
            }
            Ok(result) => {
                assert_eq!(result.name(), "Category2");
            }
        }

        let result = test_tree.search("Node1");
        match result {
            Err(e) => {
                panic!("{}", e)
            }
            Ok(result) => {
                assert_eq!(result.name(), "Node1");
                assert_eq!(result.index().index(), 0);
            }
        }
    }

    #[test]
    #[should_panic(expected = "MiddleNodes don't have indices")]
    fn test_get_index_panic() {
        get_test_tree().index();
    }

    #[test]
    fn test_deserialize() {
        let f = File::open("test_res/test.json").unwrap();
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
        assert_eq!(result.name(), "Category2");

        let result = &test_tree["Node1"];
        assert_eq!(result.name(), "Node1");
        assert_eq!(result.index().index(), 0);
    }

    #[test]
    #[should_panic(expected = "There is no node with the name \"test\" in the tree")]
    fn test_index_panic() {
        let _ = &get_test_tree()["test"];
    }
}
