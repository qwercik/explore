#[derive(Debug)]
pub struct Tree<T> {
    value: T,
    children: Vec<Box<Tree<T>>>
}

impl<T> Tree<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            children: vec![]
        }
    }

    fn append_child_tree(&mut self, child: Self) -> &mut Tree<T> {
        self.children.push(Box::new(child));
        &mut **self.children.last_mut().unwrap()
    }

    fn append_child(&mut self, value: T) -> &mut Tree<T> {
        self.append_child_tree(Tree::new(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_creating_tree() {
        let tree = Tree::new(997);
        assert_eq!(tree.value, 997);
        assert_eq!(tree.children.len(), 0);
    }

    #[test]
    fn test_appending_values() {
        let mut tree = Tree::new(997);
        let left = tree.append_child(112);
        left.append_child(123);
        left.append_child(444);
        left.append_child(23);

        let right = tree.append_child(1410);
        right.append_child(777);
        right.append_child(778);

        // Use cargo test -- --show-output to see it,
        println!("{:#?}", tree);
    }
}
