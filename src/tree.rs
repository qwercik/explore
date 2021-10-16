pub type TreeResult<T> = Result<T, TreeError>;

#[derive(Debug)]
pub enum TreeError {
    NodeIsEmpty
}

#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    NonEmpty {
        value: T,
        children: Vec<Box<Tree<T>>>
    }
}

impl<T> Tree<T> {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn new(value: T) -> Self {
        Self::NonEmpty {
            value,
            children: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn append_child_tree(&mut self, child: Self) -> TreeResult<&mut Tree<T>> {
        if let Tree::NonEmpty { children, .. } = self {
            children.push(Box::new(child));
            Ok(&mut **children.last_mut().unwrap())
        } else {
            Err(TreeError::NodeIsEmpty)
        }
    }

    pub fn append_child(&mut self, value: T) -> TreeResult<&mut Tree<T>> {
        self.append_child_tree(Tree::new(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_creating_empty_tree() {
        let tree: Tree<u32> = Tree::empty();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_creating_tree_with_root() {
        let tree = Tree::new(997);
        assert!(!tree.is_empty());

        match tree {
            Tree::NonEmpty { value, children } => {
                assert_eq!(value, 997);
                assert_eq!(children.len(), 0);
            },
            _ => panic!("Expected NonEmpty, got empty")
        }
    }

    #[test]
    fn test_appending_values() {
        fn func() -> TreeResult<Tree<u32>> {
            let mut tree = Tree::new(997);
            tree.append_child(112)?;
            let node = tree.append_child(1410).unwrap();
            let _ = node.append_child(777)?;
            let _ = node.append_child(778)?;

            Ok(tree)
        }

        let result = func();
        assert!(result.is_ok());

        let tree = result.unwrap();
        // Use cargo test -- --show-output to see it,
        println!("{:#?}", tree);
    }
}