/// The menu trees represent nodes in the delta-v map and the categories they are put into
pub trait MenuTree: PartialEq<Box<dyn MenuTree>> {
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

    /// Compares two menuTrees and returns true if they're equal
    fn eq(&self, rhs: &dyn MenuTree) -> bool;
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

fn from_yaml(yaml: &serde_yaml::Mapping) -> Box<dyn MenuTree>{
    fn read_middle(yaml: &serde_yaml::Mapping) -> Box<MiddleNode> {
        let name = yaml.get("name").unwrap().as_str().unwrap().to_string();
        let children_yaml = yaml.get("children").unwrap().as_sequence().unwrap();

        let mut children: Vec<Box<dyn MenuTree>> = vec![];
        for child in children_yaml {
            let child = child.as_mapping().unwrap();
            if yaml.contains_key("children") {
                children.push(read_middle(child));
            } else {
                children.push(read_end(child));
            }
        }

        Box::new(MiddleNode {name, children})
    }
    fn read_end(yaml: &serde_yaml::Mapping) -> Box<EndNode> {
        let name = yaml.get("name").unwrap().as_str().unwrap().to_string();
        let id = yaml.get("id").unwrap().as_u64().unwrap() as usize;

        Box::new(EndNode{name, id})
    }

    if yaml.contains_key("children") {
        read_middle(yaml)
    } else {
        read_end(yaml)
    }
}

#[cfg(test)]
mod tests {
    use crate::menutree::*;

    use std::fs::File;

    use serde::*;

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

    #[test]
    fn test_yaml() {
        let file = File::open("src/res/test.yaml");
        let yaml: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();

        let yaml = yaml.as_mapping().unwrap();
        let nodes = yaml.get("nodes").unwrap().as_mapping().unwrap();
    }
}
