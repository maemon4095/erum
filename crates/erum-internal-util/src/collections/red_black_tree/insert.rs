use super::*;

use crate::collections::util::{
    iter::{Borrowed, Inserted, Replaced},
    rc::create_rc_slice_from_iter,
};

pub enum InsertResult<E, N> {
    Keep(N),
    Split(N, E, N),
}

pub enum InsertHandResult<E: SortedMapEntry> {
    Keep(HandNode<E>),
    Split(Node<E>, E, Node<E>, E, Node<E>),
}

pub fn insert_node<E: SortedMapEntry>(node: &Node<E>, entry: E) -> InsertResult<E, Node<E>> {
    match node {
        Node::Internal(node) => insert_internal(node, entry),
        Node::Leaf(node) => match insert_leaf(node, entry) {
            InsertResult::Keep(l) => InsertResult::Keep(Node::Leaf(l)),
            InsertResult::Split(l, e, r) => InsertResult::Split(Node::Leaf(l), e, Node::Leaf(r)),
        },
    }
}

fn insert_internal<E: SortedMapEntry>(
    node: &InternalNodeInner<E>,
    entry: E,
) -> InsertResult<E, Node<E>> {
    let comparison = entry.key().cmp(&node.entry.key());
    match comparison {
        Ordering::Equal => {
            let inner = InternalNodeInner {
                entry,
                left: node.left.clone(),
                right: node.right.clone(),
            };
            InsertResult::Keep(Node::Internal(Rc::new(inner)))
        }
        Ordering::Less => insert_internal_left(node, entry),
        Ordering::Greater => insert_internal_right(node, entry),
    }
}

fn insert_internal_left<E: SortedMapEntry>(
    node: &InternalNodeInner<E>,
    entry: E,
) -> InsertResult<E, Node<E>> {
    match (&node.left, &node.right) {
        (InternalNodeBranch::Node(l), r) => {
            let new_node = match insert_node(l, entry) {
                InsertResult::Keep(l) => Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: l.into(),
                    right: r.clone(),
                })),
                InsertResult::Split(ll, mid, lr) => {
                    let new_l = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: mid,
                        left: ll,
                        right: lr,
                    }));

                    Node::Internal(Rc::new(InternalNodeInner {
                        entry: node.entry.clone(),
                        left: new_l,
                        right: r.clone(),
                    }))
                }
            };
            InsertResult::Keep(new_node)
        }
        (InternalNodeBranch::Hand(l), InternalNodeBranch::Node(r)) => {
            let new_node = match insert_hand(l, entry) {
                InsertHandResult::Keep(l) => Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: InternalNodeBranch::Hand(l),
                    right: r.clone().into(),
                })),
                InsertHandResult::Split(ll, er, lr, mid, rl) => {
                    let new_l = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: er,
                        left: ll,
                        right: lr,
                    }));

                    let new_r = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: node.entry.clone(),
                        left: rl,
                        right: r.clone(),
                    }));

                    Node::Internal(Rc::new(InternalNodeInner {
                        entry: mid,
                        left: new_l,
                        right: new_r,
                    }))
                }
            };
            InsertResult::Keep(new_node)
        }
        (InternalNodeBranch::Hand(l), InternalNodeBranch::Hand(r)) => match insert_hand(l, entry) {
            InsertHandResult::Keep(l) => {
                let new_node = Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: InternalNodeBranch::Hand(l),
                    right: InternalNodeBranch::Hand(r.clone()),
                }));

                InsertResult::Keep(new_node)
            }
            InsertHandResult::Split(ll, el, lr, mid, rl) => {
                let new_l = Node::Internal(Rc::new(InternalNodeInner {
                    entry: el,
                    left: ll.into(),
                    right: lr.into(),
                }));
                let new_r = Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: rl.into(),
                    right: InternalNodeBranch::Hand(r.clone()),
                }));

                InsertResult::Split(new_l, mid, new_r)
            }
        },
    }
}

fn insert_internal_right<E: SortedMapEntry>(
    node: &InternalNodeInner<E>,
    entry: E,
) -> InsertResult<E, Node<E>> {
    match (&node.left, &node.right) {
        (l, InternalNodeBranch::Node(r)) => {
            let new_node = match insert_node(r, entry) {
                InsertResult::Keep(r) => Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: l.clone(),
                    right: r.into(),
                })),
                InsertResult::Split(rl, mid, rr) => {
                    let new_r = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: mid,
                        left: rl,
                        right: rr,
                    }));

                    Node::Internal(Rc::new(InternalNodeInner {
                        entry: node.entry.clone(),
                        left: l.clone(),
                        right: new_r,
                    }))
                }
            };
            InsertResult::Keep(new_node)
        }
        (InternalNodeBranch::Node(l), InternalNodeBranch::Hand(r)) => {
            let new_node = match insert_hand(r, entry) {
                InsertHandResult::Keep(r) => Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: l.clone().into(),
                    right: InternalNodeBranch::Hand(r),
                })),
                InsertHandResult::Split(lr, mid, rl, er, rr) => {
                    let new_l = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: node.entry.clone(),
                        left: l.clone(),
                        right: lr,
                    }));

                    let new_r = InternalNodeBranch::Hand(Rc::new(HandNodeInner {
                        entry: er,
                        left: rl,
                        right: rr,
                    }));

                    Node::Internal(Rc::new(InternalNodeInner {
                        entry: mid,
                        left: new_l,
                        right: new_r,
                    }))
                }
            };
            InsertResult::Keep(new_node)
        }
        (InternalNodeBranch::Hand(l), InternalNodeBranch::Hand(r)) => match insert_hand(r, entry) {
            InsertHandResult::Keep(r) => {
                let new_node = Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: InternalNodeBranch::Hand(l.clone()),
                    right: InternalNodeBranch::Hand(r),
                }));
                InsertResult::Keep(new_node)
            }
            InsertHandResult::Split(lr, mid, rl, er, rr) => {
                let new_l = Node::Internal(Rc::new(InternalNodeInner {
                    entry: node.entry.clone(),
                    left: InternalNodeBranch::Hand(l.clone()),
                    right: lr.into(),
                }));

                let new_r = Node::Internal(Rc::new(InternalNodeInner {
                    entry: er,
                    left: rl.into(),
                    right: rr.into(),
                }));

                InsertResult::Split(new_l, mid, new_r)
            }
        },
    }
}

fn insert_hand<E: SortedMapEntry>(node: &HandNodeInner<E>, entry: E) -> InsertHandResult<E> {
    let comparison = entry.key().cmp(&node.entry.key());
    match comparison {
        Ordering::Equal => {
            let new_node = Rc::new(HandNodeInner {
                entry,
                left: node.left.clone(),
                right: node.right.clone(),
            });
            InsertHandResult::Keep(new_node)
        }
        Ordering::Less => match insert_node(&node.left, entry) {
            InsertResult::Keep(new_l) => {
                let new_node = Rc::new(HandNodeInner {
                    entry: node.entry.clone(),
                    left: new_l,
                    right: node.right.clone(),
                });
                InsertHandResult::Keep(new_node)
            }
            InsertResult::Split(l, e, r) => {
                InsertHandResult::Split(l, e, r, node.entry.clone(), node.right.clone())
            }
        },
        Ordering::Greater => match insert_node(&node.right, entry) {
            InsertResult::Keep(new_r) => {
                let new_node = Rc::new(HandNodeInner {
                    entry: node.entry.clone(),
                    left: node.left.clone(),
                    right: new_r,
                });
                InsertHandResult::Keep(new_node)
            }
            InsertResult::Split(l, e, r) => {
                InsertHandResult::Split(node.left.clone(), node.entry.clone(), l, e, r)
            }
        },
    }
}

fn insert_leaf<E: SortedMapEntry>(node: &LeafNode<E>, entry: E) -> InsertResult<E, LeafNode<E>> {
    let found = node.binary_search_by_key(&entry.key(), |e| e.key());
    match found {
        Ok(idx) => {
            let new_node = create_rc_slice_from_iter(Replaced::new(node, entry, idx));
            InsertResult::Keep(new_node)
        }
        Err(idx) if node.len() < 4 => {
            let new_node = create_rc_slice_from_iter(Inserted::new(node, entry, idx));
            InsertResult::Keep(new_node)
        }
        Err(idx) => {
            let mut iter = Inserted::new(node, entry, idx);
            let left_node = create_rc_slice_from_iter(Borrowed::new(&mut iter).take(2));
            let mid = iter.next().unwrap();
            let right_node = create_rc_slice_from_iter(iter);
            InsertResult::Split(left_node, mid, right_node)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::{RngExt, rngs::StdRng};
    use std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Debug,
    };

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Entry<E: Eq + Ord + Clone>(E, usize);

    impl<E: Eq + Ord + Clone> SortedMapEntry for Entry<E> {
        type Key<'a>
            = &'a E
        where
            Self: 'a;

        fn key(&self) -> Self::Key<'_> {
            &self.0
        }
    }

    fn create_random_case(sample_count: usize, element_count: usize) -> Vec<usize> {
        let mut rng: StdRng = rand::make_rng();

        std::iter::repeat_with(|| rng.random_range(0..sample_count))
            .take(element_count)
            .collect()
    }

    fn test_inserted_case<E: Ord + Clone + Debug>(cases: &[E]) {
        let mut inserted = BTreeMap::<E, usize>::new();
        let mut not_inserted = BTreeSet::from_iter(cases.iter().cloned());

        let mut map = InternalSortedMap::new();

        for entry in cases {
            let version = inserted.get(entry).map(|e| e + 1).unwrap_or(0);
            map = map.inserted(Entry(entry.clone(), version));
            inserted.insert(entry.clone(), version);
            not_inserted.remove(&entry);

            for (e, v) in inserted.iter() {
                assert_eq!(map.get(e), Some(&Entry(e.clone(), *v)));
            }

            for e in not_inserted.iter() {
                assert_eq!(map.get(e), None);
            }

            validate_invariant(&map);
        }
    }

    fn validate_invariant<E: SortedMapEntry>(map: &InternalSortedMap<E>) {
        let Some(node) = &map.root else {
            return;
        };

        let mut depths = BTreeSet::new();
        validate_invaliant_node(&mut depths, node, 0);

        assert!(depths.len() == 1, "depths = {:?}", depths);
    }

    fn validate_invaliant_node<E: SortedMapEntry>(
        depths: &mut BTreeSet<usize>,
        node: &Node<E>,
        depth: usize,
    ) {
        match node {
            Node::Internal(internal) => {
                validate_invaliant_internal(depths, &internal, depth);
            }
            Node::Leaf(_) => {
                depths.insert(depth);
            }
        }
    }

    fn validate_invaliant_internal<E: SortedMapEntry>(
        depths: &mut BTreeSet<usize>,
        node: &InternalNode<E>,
        depth: usize,
    ) {
        validate_invaliant_branch(depths, &node.left, depth + 1);
        validate_invaliant_branch(depths, &node.right, depth + 1);
    }

    fn validate_invaliant_branch<E: SortedMapEntry>(
        depths: &mut BTreeSet<usize>,
        branch: &InternalNodeBranch<E>,
        depth: usize,
    ) {
        match branch {
            InternalNodeBranch::Node(node) => validate_invaliant_node(depths, node, depth),
            InternalNodeBranch::Hand(node) => validate_invaliant_hand(depths, node, depth),
        }
    }

    fn validate_invaliant_hand<E: SortedMapEntry>(
        depths: &mut BTreeSet<usize>,
        node: &HandNode<E>,
        depth: usize,
    ) {
        validate_invaliant_node(depths, &node.left, depth);
        validate_invaliant_node(depths, &node.right, depth);
    }

    #[test]
    fn test_inserted_sorted_uniq() {
        test_inserted_case(&(0..1000).collect::<Vec<usize>>());
    }

    #[test]
    fn test_inserted_reversed_uniq() {
        test_inserted_case(&(0..1000).rev().collect::<Vec<usize>>());
    }

    #[test]
    fn test_inserted_unique() {
        test_inserted_case(&create_random_case(1, 1000));
    }

    #[test]
    fn test_inserted_random() {
        for _ in 0..1000 {
            test_inserted_case(&create_random_case(1000, 1000));
        }
    }

    #[test]
    fn test_inserted_random_low_sample() {
        for _ in 0..1000 {
            test_inserted_case(&create_random_case(10, 1000));
        }
    }
}
