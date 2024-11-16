use crate::storage::File;

pub struct Table<F: File> {
    file: F,
    file_number: u64,
    // filter_reader:Option<FilterBlockReader>
}