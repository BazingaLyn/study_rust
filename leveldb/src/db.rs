use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use crossbeam::sync::ShardedLock;
use crate::batch::WriteBatch;
use crate::compaction::ManualCompaction;
use crate::db_builder::InternalKeyComparator;
use crate::error::{Error, Result};
use crate::mem::MemTable;
use crate::options::{Options, ReadOptions, WriteOptions};
use crate::snapshot::Snapshot;
use crate::storage::Storage;
use crate::table_cache::TableCache;
use crate::utils::comparators::Comparator;
use crate::version::version_set::VersionSet;

pub trait DB {
    type Iterator;

    fn put(&mut self, write_opt: WriteOptions, key: &[u8], val: &[u8]) -> Result<()>;


    fn get(&self, read_opt: ReadOptions, key: &[u8]) -> Result<Option<Vec<u8>>>;


    fn iter(&self, read_opt: ReadOptions) -> Result<Self::Iterator>;


    fn delete(&self, write_opt: WriteOptions, key: &[u8]) -> Result<()>;


    fn write(&self, write_opt: WriteOptions, batch: WriteBatch) -> Result<()>;


    fn close(&mut self) -> Result<()>;


    fn destroy(&mut self) -> Result<()>;


    fn snapshot(&self) -> Arc<Snapshot>;
}

struct BatchTask {}

pub struct DBImpl<S: Storage + Clone, C: Comparator> {
    env: S,
    internal_comparator: InternalKeyComparator<C>,

    options: Arc<Options<C>>,

    db_path: String,

    db_lock: Option<S::F>,

    batch_queue: Mutex<VecDeque<BatchTask>>,

    process_batch_sem: Condvar,

    table_cache: TableCache<S, C>,

    version: VersionSet<S, C>,

    manual_compaction_queue: Mutex<VecDeque<ManualCompaction>>,

    background_work_finish_signal: Condvar,

    background_compaction_scheduled: AtomicBool,

    db_compaction: (Sender<()>, Receiver<()>),

    mem: ShardedLock<MemTable<C>>,

    im_mem: ShardedLock<Option<MemTable<C>>>,

    bg_error: RwLock<Option<Error>>,

    is_shutting_down: AtomicBool,
}

impl<S: Storage + Clone + 'static, C: Comparator + 'static> DBImpl<S, C> {
    fn new(options: Options<C>, db_path: String, storage: S) -> Self {
        let o = Arc::new(options);
        let icmp = InternalKeyComparator::new(o.comparator.clone());
        Self {
            env: storage.clone(),
            internal_comparator: icmp.clone(),
            options: o.clone(),
            db_path:db_path.clone(),
            db_lock: None,
            batch_queue: Mutex::new(VecDeque::new()),
            process_batch_sem: Condvar::new(),
            table_cache: TableCache::new(
                db_path.clone(),
                o.clone(),
                o.tab

            ),
            version: VersionSet {},
            manual_compaction_queue: Mutex::new(Default::default()),
            background_work_finish_signal: Default::default(),
            background_compaction_scheduled: Default::default(),
            db_compaction: ((), ()),
            mem: (),
            im_mem: (),
            bg_error: Default::default(),
            is_shutting_down: Default::default(),
        }
    }
}