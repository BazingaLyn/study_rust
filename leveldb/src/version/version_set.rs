use crate::leveldb::storage::Storage;
use crate::leveldb::utils::comparators::Comparator;

pub struct VersionSet<S: Storage + Clone, C: Comparator> {}