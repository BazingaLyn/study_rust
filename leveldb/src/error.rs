use quick_error::quick_error;

quick_error! {

    #[derive(Debug)]
    pub enum Error {
        NotFound(hint:Option<String>) {
            display("key seeking failed: {:?}", hint);
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;