pub mod error;

pub use self::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
