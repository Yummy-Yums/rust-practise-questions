/*
    Create a Binary tree and have a method add on it to add
    elements to it and min and max to find the minimum and maximum element in it.
 */

pub mod binary_tree_program {
    use std::cmp::max;

    #[derive(PartialEq, Debug)]
    pub struct Node {
        data: i8,
        left_child: Option<Box<Node>>,
        right_child: Option<Box<Node>>,
    }

    impl Node {
        pub fn new(input_data: i8) -> Self {
            Node {
                data: input_data,
                left_child: None,
                right_child: None,
            }
        }

        pub fn insert(&mut self, data: i8) -> () {
            if data < self.data {
                match &mut self.left_child {
                    Some(left_child) => left_child.insert(data),
                    None => self.left_child = Some(Box::new(Node::new(data)))
                }
            } else {
                match &mut self.right_child {
                    Some(right_child) => right_child.insert(data),
                    None => self.right_child = Some(Box::new(Node::new(data)))
                }
            }
        }

        pub fn max(&self) -> i8 {
            match &self.right_child {
                Some(right) => right.max(),
                None =>  self.data
            }
        }

        pub fn min(&self) -> i8{
            match &self.left_child {
                Some(left) => left.min(),
                None =>  self.data
            }
        }
    }

}

mod test_binary_tree {

    #[test]
    pub fn test_binary_tree() {
        let mut root = crate::chp4::binary_tree_program::binary_tree_program::Node::new(10);

        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(17);

        assert_eq!(root.max(), 17);
        assert_eq!(root.min(), 3);
    }
}