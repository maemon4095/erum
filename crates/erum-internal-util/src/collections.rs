mod red_black_tree;
mod util;

pub use red_black_tree::InternalSortedMap;

pub trait SortedMapEntry: Clone {
    type Key<'a>: Ord
    where
        Self: 'a;

    fn key(&self) -> Self::Key<'_>;
}
