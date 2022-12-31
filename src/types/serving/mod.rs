
use super::*;
use crate::internals::priority_queue::*;
use ordered_float::OrderedFloat;
use std::{cmp::Reverse, collections::BinaryHeap};

pub trait AnnoyIndexSearchApi {
    fn get_item_vector(&self, item_index: u64) -> Vec<f32>;
    fn get_nearest(
        &self,
        query_vector: &[f32],
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
    fn get_nearest_to_item(
        &self,
        item_index: u64,
        n_results: usize,
        search_k: i32,
        should_include_distance: bool,
    ) -> AnnoyIndexSearchResult;
}
