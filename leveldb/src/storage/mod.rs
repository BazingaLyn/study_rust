use crate::error::Result;
pub trait Storage: Send + Sync {

    type F: File + 'static;
}


pub trait File: Send + Sync {

    fn write(&mut self, buf: &[u8])-> Result<usize>;
}

