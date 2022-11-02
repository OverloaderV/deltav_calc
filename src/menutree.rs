use serde::{Serialize, Deserialize};

/// The menu trees represent nodes in the delta-v map and the categories they are put into
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum MenuTree {
    /// A node representing a category other nodes can be put into
    MiddleNode { name: String, children: Vec<MenuTree> },

    /// A node holding an id to be used in the graph
    EndNode { name: String, id: usize },
}

impl MenuTree {
    /// Gets the id of the node. if it's a middle node it panics
    fn get_id(&self) -> usize {
        match self {
            MenuTree::MiddleNode { .. } => {
                panic!("MiddleNodes don't have ids");
            }
            MenuTree::EndNode { id, .. } => {
                id.to_owned()
            }
        }
    }

    fn get_name(&self) -> &str {
        match self {
            MenuTree::MiddleNode { name, .. } |
            MenuTree::EndNode { name, .. } => {
                name
            }
        }
    }

    fn search_by_id(&self, search_id: usize) -> Option<&MenuTree> {
        match self {
            MenuTree::MiddleNode { children, .. } => {
                for child in children {
                    let result = child.search_by_id(search_id);
                    match result {
                        None => {}
                        Some(_) => { return result; }
                    }
                }

                None
            }
            MenuTree::EndNode { id, .. } => {
                if id.to_owned() == search_id {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    fn search_by_name(&self, search_name: &str) -> Option<&MenuTree> {
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
                    let result = child.search_by_name(search_name);

                    match result {
                        None => {}
                        Some(_) => { return result; }
                    }
                }

                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::menutree::MenuTree;
    use crate::menutree::MenuTree::{EndNode, MiddleNode};

    fn get_test_tree() -> MenuTree {
        MiddleNode {
            name: String::from("Category1"),
            children: vec![
                MiddleNode {
                    name: String::from("Category2"),
                    children: vec![
                        EndNode { name: String::from("Node1"), id: 0 },
                        EndNode { name: String::from("Node2"), id: 1 },
                    ],
                },
                EndNode { name: String::from("Node3"), id: 2 },
                EndNode { name: String::from("Node4"), id: 3 },
            ],
        }
    }

    #[test]
    fn test_search_by_id() {
        let test_tree = get_test_tree();

        let result = test_tree.search_by_id(1);
        match result {
            None => {
                panic!("The \"Node2\" node should be found");
            }
            Some(result) => {
                assert_eq!(result.get_name(), "Node2", "\"Node2\" should be found, not \"{}\"", result.get_name());
            }
        }

        let result = test_tree.search_by_id(1000);
        match result {
            None => {}
            Some(tree) => {
                panic!("No node should be found, but node \"{}\" was found", tree.get_name());
            }
        }
    }

    #[test]
    fn test_search_by_name() {
        let test_tree = get_test_tree();

        let result = test_tree.search_by_name("Category1");
        match result {
            None => {
                panic!("The \"Category1\" node should be found");
            }
            Some(result) => {
                assert_eq!(result.get_name(), "Category1", "\"Category1\" should be found, not \"{}\"", result.get_name());
            }
        }

        let result = test_tree.search_by_name("Node3");
        match result {
            None => {
                panic!("The \"Node3\" node should be found");
            }
            Some(result) => {
                assert_eq!(result.get_id(), 2, "\"Node3\" should be found, not \"{}\"", result.get_name());
            }
        }

        let result = test_tree.search_by_name("test");
        match result {
            None => {}
            Some(tree) => {
                panic!("No node should be found, but node \"{}\" was found", tree.get_name());
            }
        }
    }

    #[test]
    #[should_panic(expected="MiddleNodes don't have ids")]
    fn test_id_panic() {
        get_test_tree().get_id();
    }

    #[test]
    fn test_deserialize() {
        let file = File::open("res/test.json").unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        let json = json.get("nodes").unwrap();
        let tree: MenuTree = serde_json::from_value(json.to_owned()).unwrap();

        assert_eq!(get_test_tree(), tree, "The tree hasn't been deserialized properly");
    }
}
