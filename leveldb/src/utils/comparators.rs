use std::cmp::Ordering;

pub(crate) trait Comparator: Send + Sync + Default + Clone {
    fn compare(&self, a: &[u8], b: &[u8]) -> Ordering;


    fn name(&self) -> &str;


    fn separator(&self, a: &[u8], b: &[u8]) -> Vec<u8>;


    fn successors(&self, a: &[u8], b: &[u8]) -> Vec<u8>;
}