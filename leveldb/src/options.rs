use num_derive::FromPrimitive;
use crate::snapshot::Snapshot;
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

    pub paranoid_checks: bool,

    pub max_levels: usize,

    pub l0_compaction_threshold: usize,

    pub l0_slowdown_writes_threshold: usize,

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
