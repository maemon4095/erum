use super::*;

pub fn search_node<'a, E: SortedMapEntry>(node: &'a Node<E>, key: E::Key<'a>) -> Option<&'a E> {
    match node {
        Node::Internal(node) => search_internal(node, key),
        Node::Leaf(node) => search_leaf(node, key),
    }
}

fn search_leaf<'a, E: SortedMapEntry>(node: &'a LeafNode<E>, key: E::Key<'a>) -> Option<&'a E> {
    node.binary_search_by_key(&key, |e| e.key())
        .ok()
        .map(|i| &node[i])
}

fn search_internal<'a, E: SortedMapEntry>(
    node: &'a InternalNode<E>,
    key: E::Key<'a>,
) -> Option<&'a E> {
    match key.cmp(&node.entry.key()) {
        Ordering::Less => search_branch(&node.left, key),
        Ordering::Equal => Some(&node.entry),
        Ordering::Greater => search_branch(&node.right, key),
    }
}

fn search_branch<'a, E: SortedMapEntry>(
    branch: &'a InternalNodeBranch<E>,
    key: E::Key<'a>,
) -> Option<&'a E> {
    match branch {
        InternalNodeBranch::Node(node) => search_node(node, key),
        InternalNodeBranch::Hand(node) => search_hand(node, key),
    }
}

fn search_hand<'a, E: SortedMapEntry>(node: &'a HandNode<E>, key: E::Key<'a>) -> Option<&'a E> {
    match key.cmp(&node.entry.key()) {
        Ordering::Less => search_node(&node.left, key),
        Ordering::Equal => Some(&node.entry),
        Ordering::Greater => search_node(&node.right, key),
    }
}
