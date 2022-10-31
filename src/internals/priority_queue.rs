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
    fn eq(&self, other: &Self) -> bool {
        self.ord == other.ord
    }
}

impl<I, O> Ord for BinaryHeapItem<I, O>
where
    I: Eq,
    O: PartialEq + Eq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<I, O> PartialOrd for BinaryHeapItem<I, O>
where
    I: Eq,
    O: PartialEq + Eq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ord.partial_cmp(&other.ord)
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, fmt::Debug};

    pub struct PriorityQueue<K, P>
    where
        P: PartialOrd,
    {
        ord: Ordering,
        keys: Vec<K>,
        priorities: Vec<P>,
    }

    impl<K, P> PriorityQueue<K, P>
    where
        P: PartialOrd + Debug,
    {
        pub fn with_capacity(capacity: usize, reverse: bool) -> PriorityQueue<K, P> {
            PriorityQueue {
                keys: Vec::with_capacity(capacity),
                priorities: Vec::with_capacity(capacity),
                ord: match reverse {
                    true => Ordering::Greater,
                    _ => Ordering::Less,
                },
            }
        }

        pub fn push(&mut self, key: K, priority: P) {
            self.keys.push(key);
            self.priorities.push(priority);
            let pos = self.priorities.len() - 1;
            if pos > 0 && !self.max_heap_up_adjust(pos) {
                self.keys.pop();
                self.priorities.pop();
            }
        }

        fn max_heap_up_adjust(&mut self, position: usize) -> bool {
            let mut pos = position;
            let priorities = self.priorities.as_mut_slice();
            let keys = self.keys.as_mut_slice();
            while pos > 0 {
                let p_pos = (pos - 1) / 2;
                match priorities[p_pos].partial_cmp(&priorities[pos]) {
                    None => return false,
                    Some(ord) if ord == self.ord => {
                        priorities.swap(pos, p_pos);
                        keys.swap(pos, p_pos);
                        pos = p_pos;
                    }
                    _ => return true,
                }
            }
            true
        }

        fn max_heap_down_adjust(&mut self, position: usize) -> bool {
            let len = self.len();
            let mut pos = position;
            let priorities = self.priorities.as_mut_slice();
            let keys = self.keys.as_mut_slice();
            loop {
                let lc_pos = pos * 2 + 1;
                let rc_pos = pos * 2 + 2;
                let mut largest_pos = pos;
                if lc_pos < len {
                    largest_pos = match priorities[largest_pos].partial_cmp(&priorities[lc_pos]) {
                        None => return false,
                        Some(ord) if ord == self.ord => lc_pos,
                        _ => largest_pos,
                    };
                }
                if rc_pos < len {
                    largest_pos = match priorities[largest_pos].partial_cmp(&priorities[rc_pos]) {
                        None => return false,
                        Some(ord) if ord == self.ord => rc_pos,
                        _ => largest_pos,
                    };
                }
                if largest_pos != pos {
                    priorities.swap(largest_pos, pos);
                    keys.swap(largest_pos, pos);
                    pos = largest_pos;
                } else {
                    break;
                }
            }
            true
        }

        // fn max_heap_build(&mut self) {
        //     let len = self.len();
        //     let priorities = self.priorities.as_mut_slice();
        //     let keys = self.keys.as_mut_slice();
        //     for pos in (0..len / 2).rev() {
        //         let lc_pos = pos * 2 + 1;
        //         let rc_pos = pos * 2 + 2;
        //         let mut largest_pos = pos;
        //         if lc_pos < len {
        //             largest_pos = match priorities[largest_pos].partial_cmp(&priorities[lc_pos]) {
        //                 None => return,
        //                 Some(Ordering::Less) => lc_pos,
        //                 _ => largest_pos,
        //             };
        //         }
        //         if rc_pos < len {
        //             largest_pos = match priorities[largest_pos].partial_cmp(&priorities[rc_pos]) {
        //                 None => return,
        //                 Some(Ordering::Less) => rc_pos,
        //                 _ => largest_pos,
        //             };
        //         }
        //         if largest_pos != pos {
        //             priorities.swap(largest_pos, pos);
        //             keys.swap(largest_pos, 