use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug, Clone, Eq)]
pub struct BinaryHeapItem<I, O>
where
    I: Eq,
    O: PartialEq + Eq + PartialOrd,
{
    pub item: I,
    pub ord: O,
}

impl<I, O> PartialEq for BinaryHeapItem<I, O>
where
    I: Eq,
    O: PartialEq + Eq + PartialOrd,
{
    fn eq(&