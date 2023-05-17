use crate::binary_tree::TreeNode;
fn create_tree() -> TreeNode {
    let mut tree = TreeNode::new(15);
    tree.insert(6);
    tree.insert(18);
    tree.insert(3);
    tree.insert(7);
    tree.insert(17);
    tree.insert(20);
    tree.insert(2);
    tree.insert(4);
    tree.insert(13);
    tree.insert(9);
    tree
}

fn create_tree_2() -> TreeNode {
    let mut tree = TreeNode::new(15);
    tree.insert(6);
    tree.insert(18);
    tree.insert(7);
    tree.insert(17);
    tree.insert(4);
    tree.insert(2);
    tree.insert(13);
    tree.insert(9);
    tree
}

#[test]
fn test_search_recursive() {
    let tree = create_tree();
    assert_eq!(tree.search_recursive(13).unwrap().value, 13);
    assert_eq!(tree.search_recursive(18).unwrap().value, 18);
    assert_eq!(tree.search_recursive(2).unwrap().value, 2);
    assert_eq!(tree.search_recursive(15).unwrap().value, 15);
    assert_eq!(tree.search_recursive(100), None);
}

#[test]
fn test_search() {
    let tree = create_tree();
    assert_eq!(tree.search(13).unwrap().value, 13);
    assert_eq!(tree.search(18).unwrap().value, 18);
    assert_eq!(tree.search(2).unwrap().value, 2);
    assert_eq!(tree.search(15).unwrap().value, 15);
    assert_eq!(tree.search(100), None);
}

#[test]
fn test_minimum_and_maximum() {
    let tree = create_tree();
    assert_eq!(tree.minimum().value, 2);
    assert_eq!(tree.maximum().value, 20);
}

#[test]
fn test_successor_and_predecessor() {
    let tree = create_tree();
    assert_eq!(tree.predecessor(3).unwrap().value, 2);
    assert_eq!(tree.successor(3).unwrap().value, 4);
    assert_eq!(tree.predecessor(2), None);
    assert_eq!(tree.successor(20), None);
}

#[test]
fn test_parent() {
    let tree = create_tree();
    assert_eq!(tree.parent(2).unwrap().value, 3);
}

#[test]
fn test_delete() {
    let mut tree = create_tree();
    let tree_2 = create_tree_2();
    tree.delete(3);
    tree.delete(20);
    assert_eq!(tree, tree_2);
}

#[test]
fn test_height_recursive() {
    let tree = create_tree();
    let tree_2 = TreeNode::new(1);
    assert_eq!(tree.height_recursive(), 5);
    assert_eq!(tree_2.height_recursive(), 1);
}

#[test]
fn test_height() {
    let tree = create_tree();
    let tree_2 = TreeNode::new(1);
    assert_eq!(tree.height(), 5);
    assert_eq!(tree_2.height(), 1);
}
