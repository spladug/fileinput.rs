//! Types which can be used to tune the behavior of `FileInput`.
//!
//! A default strategy is provided.

use std;
use std::fmt;

pub type DefaultIoStrategy = IoUseStd;

#[derive(Debug, Default)]
pub struct IoUseStd;

pub trait IoStrategy: Default + fmt::Debug {
    fn open<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<Box<std::io::Read>>;
    fn stdin(&self) -> Box<std::io::Read>;
}

impl IoStrategy for IoUseStd {
    #[inline]
    fn open<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<Box<std::io::Read>> {
        Ok(Box::new(try!(std::fs::File::open(path))))
    }

    #[inline]
    fn stdin(&self) -> Box<std::io::Read> {
        Box::new(std::io::stdin())
    }
}
