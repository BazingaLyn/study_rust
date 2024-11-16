use crate::snapshot::Snapshot;
use crate::storage::Storage;
use crate::utils::comparators::Comparator;

pub struct VersionSet<S: Storage + Clone, C: Comparator> {

    // pub snapshots: SnapshotList,
}

impl<S:Storage + Clone +'static,C:Comparator + 'static> VersionSet<S, C> {
    pub fn new() -> VersionSet<S, C> {
        todo!()
    }
}