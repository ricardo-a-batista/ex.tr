use std::io;

use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    IO(io::Error),
}
