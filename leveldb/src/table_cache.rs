use std::sync::Arc;
use crate::cache::Cache;
use crate::options::Options;
use crate::sstable::table::Table;
use crate::storage::Storage;
use crate::utils::comparators::Comparator;

// tableCache 是用来缓存sst和sstable
pub struct TableCache<S: Storage + Clone, C: Comparator> {
    storage: S,

    db_path: String,

    options: Arc<Options<C>>,

    cache: Arc<dyn Cache<u64, Arc<Table<S::F>>>>,
}

impl<S: Storage + Clone, C: Comparator + 'static> TableCache<S, C> {
    pub fn new(db_path: String, options: Arc<Options<C>>, size: usize, storage: S) -> Self {
        todo!()
    }
}