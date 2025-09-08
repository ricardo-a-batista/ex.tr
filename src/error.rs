use std::io;

use derive_more::{Display, From};

#[derive(Display, Debug, From)]
pub enum Error {
    #[from]
    #[display("IO error: {_0}")]
    Io(io::Error),
}

impl std::error::Error for Error {}

