pub mod buffer_pool;
pub mod page;
pub mod record;
pub mod storage_engine;

pub mod error {
    pub enum Error {
        IO(std::io::Error),
        Bincode(bincode::Error),
        PageSizeExceeded(usize),
    }

    impl From<std::io::Error> for Error {
        fn from(io_error: std::io::Error) -> Self {
            Self::IO(io_error)
        }
    }

    impl From<bincode::Error> for Error {
        fn from(bincode_error: bincode::Error) -> Self {
            Self::Bincode(bincode_error)
        }
    }
}
