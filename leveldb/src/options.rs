use std::sync::Arc;
use log::LevelFilter;
use num_derive::FromPrimitive;
use crate::cache::Cache;
use crate::filter::FilterPolicy;
use crate::snapshot::Snapshot;
use crate::sstable::block::Block;
use crate::utils::comparators::Comparator;

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum CompressionType {
    NoCompression = 0,
    SnappyCompression = 1,
    Unknown = 2,
}

impl From<u8> for CompressionType {
    fn from(val: u8) -> Self {
        num_traits::FromPrimitive::from_u8(val).unwrap()
    }
}


#[derive(Clone)]
pub struct Options<C: Comparator> {
    pub comparator: C,

    pub create_if_missing: bool,

    pub error_if_exists: bool,
    //todo
    pub paranoid_checks: bool,

    pub max_levels: usize,

    pub l0_compaction_threshold: usize,

    pub l0_slowdown_writes_threshold: usize,

    pub l0_stop_writes_threshold: usize,

    pub l1_max_bytes: usize,

    pub max_mem_compact_level: usize,

    pub read_bytes_period: usize,

    pub write_buffer_size: usize,

    pub max_open_files: usize,

    pub block_cache: Option<Arc<dyn Cache<Vec<u8>, Arc<Block>>>>,

    pub non_table_cache_file: usize,

    pub block_size: usize,

    pub block_restart_interval: usize,

    pub max_file_size: usize,

    pub compression: CompressionType,

    pub reuse_logs: bool,

    pub filter_policy: Option<Arc<dyn FilterPolicy>>,

    pub logger: Options<slog::Logger>,

    pub logger_level: LevelFilter,
}

impl <C:Comparator> Options<C> {
    pub(crate) fn table_cache_size(&self) -> usize {
        self.max_file_size - self.non_table_cache_file
    }
}


#[derive(Default)]
pub struct WriteOptions {
    pub sync: bool,
}


#[derive(Copy, Clone)]
pub struct ReadOptions {
    pub verify_checksums: bool,

    pub fill_cache: bool,

    pub snapshot: Option<Snapshot>,
}

impl Default for ReadOptions {
    fn default() -> Self {
        ReadOptions {
            verify_checksums: false,
            fill_cache: true,
            snapshot: None,
        }
    }
}
