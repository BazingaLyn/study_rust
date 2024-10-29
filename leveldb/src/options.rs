use crate::leveldb::snapshot::Snapshot;

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
