#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
pub struct Snapshot {
    sequence_number: u64,
}