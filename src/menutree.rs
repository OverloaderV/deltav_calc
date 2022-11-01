/// The menu trees represent nodes in the delta-v map and the categories they are put into
pub trait MenuTree {
    /// Get the id of the node
    ///
    /// If it's a node without id, return the max value
    fn get_id(&self) -> usize;

    /// Get the name of the node
    fn get_name(&self) -> &str;

    /// Get the sub-node with the given id
    fn search_by_id(&self, id: usize) -> Option<&dyn MenuTree>;

    /// Get the sub-node with the given name
    fn search_by_name(&self, name: &str) -> Option<&dyn MenuTree>;
}

pub struct MiddleNode {
    name: String,
    children: Vec<Box<dyn MenuTree>>,
}

impl MenuTree for MiddleNode {
    fn get_id(&self) -> usize {
        return usize::MAX;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn search_by_id(&self, id: usize) -> Option<&dyn MenuTree> {
        for child in &self.children {
            match child.search_by_id(id) {
                None => {
                    continue;
                }
                Some(value) => {
                    return Some(value);
                }
            }
        }

        None
    }

    fn search_by_name(&self, name: &str) -> Option<&dyn MenuTree> {
        if self.name == name {
            return Some(self);
        }

        for child in &self.children {
            match child.search_by_name(name) {
                None => {
                    continue;
                }
                Some(value) => {
                    return Some(value);
                }
            }
        }

        None
    }
}

pub struct EndNode {
    name: String,
    id: usize,
}

impl MenuTree for EndNode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn search_by_id(&self, id: usize) -> Option<&dyn MenuTree> {
        if self.id == id {
            Some(self)
        } else {
            None
        }
    }

    fn search_by_name(&self, name: &str) -> Option<&dyn MenuTree> {
        if self.name == name {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::menutree::*;

    fn get_test_tree() -> Box<dyn MenuTree> {
        Box::new(MiddleNode {
            name: String::from("Category1"),
            children: vec![
                Box::new(MiddleNode {
                    name: String::from("Category2"),
                    children: vec![
                        Box::new(EndNode { name: String::from("Node1"), id: 0 }),
                        Box::new(EndNode { name: String::from("Node2"), id: 1 }),
                    ],
                }),
                Box::new(EndNode { name: String::from("Node3"), id: 2 }),
                Box::new(EndNode { name: String::from("Node4"), id: 3 }),
            ],
        })
    }

    #[test]
    fn test_id_search() {
        let test_tree = get_test_tree();

        let result = test_tree.search_by_id(1);
        assert!(result.is_some(), "The result should be the \"Node2\" Node");
        let result = result.unwrap();
        assert_eq!(result.get_name(), "Node2", "The result should be the \"Node2\" Node");

        let result = test_tree.search_by_id(100);
        assert!(result.is_none(), "There should be no node with the id 100");
    }

    #[test]
    fn test_name_search() {
        let test_tree = get_test_tree();

        let result = test_tree.search_by_name("Node1");
        assert!(result.is_some(), "The result should be the Node with id 0");
        let result = result.unwrap();
        assert_eq!(result.get_id(), 0, "The result should be the Node with the id 0");

        let result = test_tree.search_by_name("Doesn't exist");
        assert!(result.is_none(), "There should be no node with the name \"Doesn't exist\"");

        let result = test_tree.search_by_name("Category1");
        assert!(result.is_some(), "The result should be the Category1 Node");
        let result = result.unwrap();
        assert_eq!(result.get_id(), usize::MAX, "The id of the Category1 Node should be {}", usize::MAX);
    }
}
