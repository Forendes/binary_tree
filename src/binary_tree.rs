use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn search_recursive(&self, key: i32) -> Option<&Self> {
        if self.value == key {
            return Some(self);
        }
        if self.value > key {
            self.left.as_ref().and_then(|x| x.search_recursive(key))
        } else {
            self.right.as_ref().and_then(|x| x.search_recursive(key))
        }
    }

    pub fn search(&self, key: i32) -> Option<&Self> {
        let mut current = self;
        while current.value != key {
            if key < current.value {
                current = current.left.as_ref()?;
            } else {
                current = current.right.as_ref()?;
            }
        }
        Some(current)
    }

    pub fn minimum(&self) -> &Self {
        let mut current = self;
        while let Some(left) = current.left.as_ref() {
            current = left;
        }
        current
    }

    pub fn maximum(&self) -> &Self {
        let mut current = self;
        while let Some(right) = current.right.as_ref() {
            current = right;
        }
        current
    }

    pub fn parent(&self, key: i32) -> Option<&Self> {
        let mut parent: Option<&TreeNode> = None;
        let mut current = self;
        while current.value != key {
            parent = Some(current);
            if current.value > key {
                current = current.left.as_ref()?;
            } else {
                current = current.right.as_ref()?;
            }
        }
        parent
    }

    fn parent_mut(&mut self, key: i32) -> Option<(&mut Self, i32)> {
        let mut current = self;
        loop {
            if current.left.is_none() && current.right.is_none() {
                return None;
            }
            if current
                .left
                .as_ref()
                .map_or(false, |left| left.value == key)
            {
                return Some((current, 0));
            }
            if current
                .right
                .as_ref()
                .map_or(false, |right| right.value == key)
            {
                return Some((current, 1));
            }
            if current.value > key {
                current = current.left.as_mut()?;
            } else {
                current = current.right.as_mut()?;
            }
        }
    }

    // Helper function to get child as mut
    fn get_child(&mut self, side: i32) -> &mut Option<Box<Self>> {
        if side == 0 {
            &mut self.left
        } else if side == 1 {
            &mut self.right
        } else {
            unreachable!("Side should always return 0 for left child or 1 for right child")
        }
    }

    pub fn successor(&self, key: i32) -> Option<&Self> {
        let mut succesor: Option<&TreeNode> = None;
        let mut current: &TreeNode = self;
        while current.value != key {
            if current.value > key {
                succesor = Some(current);
                current = current.left.as_ref()?;
            } else {
                current = current.right.as_ref()?;
            }
        }
        if let Some(right) = &current.right {
            return Some(right.minimum());
        }
        succesor
    }

    pub fn predecessor(&self, key: i32) -> Option<&Self> {
        let mut predecessor: Option<&TreeNode> = None;
        let mut current: &TreeNode = self;
        while current.value != key {
            if current.value > key {
                current = current.left.as_ref()?;
            } else {
                predecessor = Some(current);
                current = current.right.as_ref()?;
            }
        }
        if let Some(left) = &current.left {
            return Some(left.maximum());
        }
        predecessor
    }

    pub fn insert(&mut self, key: i32) {
        let mut current: &mut TreeNode = self;
        loop {
            if current.value > key {
                if current.left.is_none() {
                    return current.left = Some(Box::new(TreeNode::new(key)));
                } else {
                    current = current.left.as_mut().unwrap()
                }
            } else if current.right.is_none() {
                return current.right = Some(Box::new(TreeNode::new(key)));
            } else {
                current = current.right.as_mut().unwrap();
            }
        }
    }

    pub fn delete(&mut self, key: i32) {
        // finding parent of target & which child is target
        let (parent, side) = match self.parent_mut(key) {
            Some((parent, side)) => (parent, side),
            // TODO: Add implementation if trying to delete root
            None => return,
        };
        // access mut target
        let target = parent.get_child(side);
        let target_mut = target.as_mut().unwrap();
        // no left subtree
        if target_mut.left.is_none() {
            let target_right = target_mut.right.take();
            *target = target_right;
        // no right subtree
        } else if target_mut.right.is_none() {
            let target_left = target_mut.left.take();
            *target = target_left;
        // both are present
        } else {
            // find min value to replace target with
            // exists because not empty
            let tree_min = target_mut.right.as_ref().unwrap().minimum().value;
            // if child is min value => min value has no left subtree
            if target_mut.right.as_ref().unwrap().value == tree_min {
                let mut target_right = target_mut.right.take();
                let target_left_saved = target_mut.left.take();
                // save left subtree of target
                target_right.as_mut().unwrap().left = target_left_saved;
                *target = target_right;
                return;
            }
            // parent of min value is not target, find it
            let (min_parent, side) = match target_mut.right.as_mut().unwrap().parent_mut(tree_min) {
                Some((min_parent, side)) => (min_parent, side),
                None => return,
            };
            let min_target = min_parent.get_child(side);
            let min_target_mut = min_target.as_mut().unwrap();
            // if min value has right subtree, save it
            let min_target_right_subtree = min_target_mut.right.take();
            let mut saved_min_target = min_target.take();
            // pop min value from the tree and move to it's place his right subtree
            *min_target = min_target_right_subtree;
            // move both subtree of target to min value
            saved_min_target.as_mut().unwrap().right = target_mut.right.take();
            saved_min_target.as_mut().unwrap().left = target_mut.left.take();
            // put in place of target saved min target
            *target = saved_min_target;
        }
    }

    pub fn height(&self) -> usize {
        let mut depth = 0;
        let mut children_queue = VecDeque::new();
        children_queue.push_back(self);
        while !children_queue.is_empty() {
            depth += 1;
            let children_num = children_queue.len();
            for _ in 0..children_num {
                let temp = children_queue.pop_front().unwrap();
                if let Some(left_child) = &temp.left {
                    children_queue.push_back(left_child.as_ref());
                }
                if let Some(right_child) = &temp.right {
                    children_queue.push_back(right_child.as_ref());
                }
            }
        }
        depth
    }

    pub fn height_recursive(&self) -> usize {
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }
        let l_height = self
            .left
            .as_ref()
            .map_or(0, |x| TreeNode::height_recursive(x));
        let r_height = self
            .right
            .as_ref()
            .map_or(0, |x| TreeNode::height_recursive(x));
        if l_height > r_height {
            l_height + 1
        } else {
            r_height + 1
        }
    }
}
