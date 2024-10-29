use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use crossbeam::sync::ShardedLock;
use crate::leveldb::batch::WriteBatch;
use crate::leveldb::compaction::ManualCompaction;
use crate::leveldb::db_builder::InternalKeyComparator;
use crate::leveldb::error::{Error, Result};
use crate::leveldb::mem::MemTable;
use crate::leveldb::options::{ReadOptions, WriteOptions};
use crate::leveldb::snapshot::Snapshot;
use crate::leveldb::storage::Storage;
use crate::leveldb::table_cache::TableCache;
use crate::leveldb::utils::comparators::Comparator;
use crate::leveldb::version::version_set::VersionSet;

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

    options: Arc<Option<C>>,

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