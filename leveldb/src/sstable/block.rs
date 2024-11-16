use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Block {
    data: Arc<Vec<u8>>,

    restart_offset: u32,

    restart_len: u32,

}

impl Block {}