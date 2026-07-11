#![allow(unused_imports)]
use crate::prelude::DynError;
use macron::{Display, Error, From};

// The application error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "Unsupported operating system")]
    UnsupportedOS,
}
