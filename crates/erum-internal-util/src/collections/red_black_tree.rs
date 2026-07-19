mod insert;
mod search;

use super::SortedMapEntry;

use std::cmp::Ordering;
use std::rc::Rc;

use insert::{InsertResult, insert_node};
use search::search_node;

#[derive(Debug)]
pub struct InternalSortedMap<E: SortedMapEntry> {
    root: Option<Node<E>>,
}

impl<E: SortedMapEntry> Default for InternalSortedMap<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: SortedMapEntry> InternalSortedMap<E> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn inserted(&self, entry: E) -> Self {
        match &self.root {
            Some(root) => {
                let new_root = match insert_node(root, entry) {
                    InsertResult::Keep(n) => n,
                    InsertResult::Split(l, e, r) => Node::Internal(Rc::new(InternalNodeInner {
                        entry: e,
                        left: InternalNodeBranch::Node(l),
                        right: InternalNodeBranch::Node(r),
                    })),
                };

                Self {
                    root: Some(new_root),
                }
            }
            None => Self {
                root: Some(Node::Leaf(Rc::new([entry]))),
            },
        }
    }

    pub fn get<'a>(&'a self, key: E::Key<'a>) -> Option<&'a E> {
        self.root.as_ref().and_then(|e| search_node(e, key))
    }
}

type InternalNode<E> = Rc<InternalNodeInner<E>>;

#[derive(Debug)]
struct InternalNodeInner<E: SortedMapEntry> {
    entry: E,
    left: InternalNodeBranch<E>,
    right: InternalNodeBranch<E>,
}

#[derive(Debug)]
enum InternalNodeBranch<E: SortedMapEntry> {
    Node(Node<E>),
    Hand(HandNode<E>),
}

impl<E: SortedMapEntry> Clone for InternalNodeBranch<E> {
    fn clone(&self) -> Self {
        match self {
            Self::Node(v) => Self::Node(v.clone()),
            Self::Hand(v) => Self::Hand(v.clone()),
        }
    }
}

impl<E: SortedMapEntry> From<Node<E>> for InternalNodeBranch<E> {
    fn from(value: Node<E>) -> Self {
        InternalNodeBranch::Node(value)
    }
}

type HandNode<E> = Rc<HandNodeInner<E>>;

#[derive(Debug)]
struct HandNodeInner<E: SortedMapEntry> {
    entry: E,
    left: Node<E>,
    right: Node<E>,
}

#[derive(Debug)]
enum Node<E: SortedMapEntry> {
    Internal(InternalNode<E>),
    Leaf(LeafNode<E>),
}

impl<E: SortedMapEntry> Clone for Node<E> {
    fn clone(&self) -> Self {
        match self {
            Self::Internal(v) => Self::Internal(v.clone()),
            Self::Leaf(v) => Self::Leaf(v.clone()),
        }
    }
}

type LeafNode<E> = Rc<[E]>; // 1～4個のエントリを持つ。本来1～3個だが、分割する際に対称にならないため4個とする。
