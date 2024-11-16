use crate::db_builder::InternalKeyComparator;
use crate::utils::comparators::Comparator;

#[derive(Clone,Default)]
pub struct KeyComparator<C:Comparator> {
    icmp:InternalKeyComparator<C>,
}

pub struct MemTable<C:Comparator> {
    cmp:KeyComparator<C>,
}